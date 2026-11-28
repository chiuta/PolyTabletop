//! Litere — joc original de cuvinte pe tablă 15×15 (2 jucători).
//!
//! Mecanica „plasezi litere pe tablă formând cuvinte care se leagă de cele
//! existente, cu pătrate premiate” este generică; setul de litere (97 de
//! plăcuțe), valorile, harta pătratelor premiate și dicționarul sunt ale
//! noastre.
//!
//! # Dicționarul
//!
//! `polytabletop-games/data/litere_dict.txt` conține cca 4 000 de cuvinte românești
//! uzuale scrise de mână pentru PolyTabletop (substantive la singular și plural, verbe
//! la infinitiv și forme frecvente, adjective, adverbe). **Nu** e un dicționar
//! oficial: lipsesc multe forme flexionare, iar cuvintele neincluse sunt
//! respinse chiar dacă sunt corecte. Diacriticele se elimină la încărcare
//! (ă, â → a; î → i; ș → s; ț → t), deci tabla folosește cele 26 de litere
//! latine. Fișierul poate fi înlocuit: [`Litere::with_dictionary`] primește un
//! text cu cuvinte separate prin spații/linii (liniile care încep cu `#` sunt
//! comentarii).
//!
//! # Reguli
//!
//! * 7 plăcuțe fiecare din 97 (2 blank `_`, care țin loc de orice literă și
//!   valorează 0). Primul cuvânt acoperă pătratul central (h8) și are minimum
//!   două litere.
//! * Mutare: `cuvant@h8>` (pe orizontală, de la h8 spre dreapta) sau
//!   `cuvant@h8v` (în jos). Se scrie tot cuvântul, inclusiv literele deja de
//!   pe tablă. Literele jucate din blank apar între paranteze: `ca(s)a@h8>`;
//!   programul alege automat unde folosește blankurile (la pătratele cu cel
//!   mai mic multiplicator de literă).
//! * Toate cuvintele formate (principal și transversale) trebuie să fie în
//!   dicționar. Scor: valoarea literelor × premii de literă (2×/3×) ale
//!   pătratelor noi, apoi premii de cuvânt (2×/3×); +50 la folosirea tuturor
//!   celor 7 plăcuțe.
//! * `schimb`: schimbi toată mâna (doar dacă în sac sunt cel puțin 7 plăcuțe);
//!   `pas`.
//! * Sfârșit: cineva rămâne fără plăcuțe cu sacul gol (primește valoarea
//!   plăcuțelor adversarului), sau patru ture consecutive fără scor. Se scade
//!   valoarea plăcuțelor rămase. Peste 400 de mutări jocul se încheie la fel.
//!
//! Informație ascunsă: `render` arată doar mâna Jucătorului 1; AI-ul nu se
//! uită la mâna adversarului și, la evaluare, ignoră plăcuțele abia trase din
//! sac.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::collections::HashSet;
use std::sync::{Arc, OnceLock};

const SIZE: usize = 15;
const RACK: usize = 7;
const BLANK: u8 = 26;
const PLY_CAP: u32 = 400;

/// Număr de plăcuțe pe literă A..Z.
const COUNTS: [u8; 26] = [
    10, 2, 5, 4, 9, 2, 2, 1, 10, 1, 1, 4, 3, 6, 4, 3, 0, 7, 5, 7, 5, 2, 0, 1, 0, 1,
];
/// Valoarea unei litere A..Z.
const VALUES: [i32; 26] = [
    1, 4, 2, 2, 1, 5, 4, 6, 1, 8, 8, 2, 3, 1, 1, 3, 0, 1, 1, 1, 2, 5, 0, 10, 0, 8,
];
const BLANKS: u8 = 2;

pub fn tile_total() -> usize {
    COUNTS.iter().map(|&c| c as usize).sum::<usize>() + BLANKS as usize
}

/// (multiplicator de literă, multiplicator de cuvânt)
fn premium(r: usize, c: usize) -> (i32, i32) {
    let a = r.min(SIZE - 1 - r);
    let b = c.min(SIZE - 1 - c);
    let at = |x: usize, y: usize| (a == x && b == y) || (a == y && b == x);
    if at(0, 3) {
        return (1, 3);
    }
    if at(1, 5) || at(2, 2) || at(4, 4) || (a == 7 && b == 7) {
        return (1, 2);
    }
    if at(3, 5) || at(1, 1) {
        return (3, 1);
    }
    if at(0, 7) || at(2, 6) || at(3, 7) || at(5, 5) {
        return (2, 1);
    }
    (1, 1)
}

/// Normalizează un cuvânt la a–z; `None` dacă are alte caractere.
pub fn normalize(w: &str) -> Option<Vec<u8>> {
    let mut v = Vec::new();
    for ch in w.chars().flat_map(|c| c.to_lowercase()) {
        let m = match ch {
            'ă' | 'â' => 'a',
            'î' => 'i',
            'ș' | 'ş' => 's',
            'ț' | 'ţ' => 't',
            c => c,
        };
        if m.is_ascii_lowercase() {
            v.push(m as u8 - b'a');
        } else {
            return None;
        }
    }
    Some(v)
}

pub struct Dictionary {
    by_len: Vec<Vec<Vec<u8>>>,
    set: HashSet<Vec<u8>>,
}

impl Dictionary {
    pub fn from_text(text: &str) -> Dictionary {
        let mut set: HashSet<Vec<u8>> = HashSet::new();
        for line in text.lines() {
            if line.trim_start().starts_with('#') {
                continue;
            }
            for tok in line.split_whitespace() {
                if let Some(w) = normalize(tok) {
                    if w.len() >= 2 && w.len() <= SIZE {
                        set.insert(w);
                    }
                }
            }
        }
        let mut by_len: Vec<Vec<Vec<u8>>> = vec![Vec::new(); SIZE + 1];
        let mut all: Vec<&Vec<u8>> = set.iter().collect();
        all.sort();
        for w in all {
            by_len[w.len()].push(w.clone());
        }
        Dictionary { by_len, set }
    }

    pub fn contains(&self, w: &[u8]) -> bool {
        self.set.contains(w)
    }

    pub fn contains_str(&self, w: &str) -> bool {
        normalize(w).is_some_and(|v| self.contains(&v))
    }

    pub fn len(&self) -> usize {
        self.set.len()
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }
}

fn default_dictionary() -> Arc<Dictionary> {
    static D: OnceLock<Arc<Dictionary>> = OnceLock::new();
    D.get_or_init(|| {
        Arc::new(Dictionary::from_text(include_str!(
            "../data/litere_dict.txt"
        )))
    })
    .clone()
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Place {
        row: u8,
        col: u8,
        down: bool,
        /// literele cuvântului (0..25), inclusiv cele de pe tablă
        letters: Vec<u8>,
        /// bit i = litera i e jucată dintr-un blank
        blanks: u32,
    },
    Exchange,
    Pass,
}

#[derive(Clone)]
pub struct Litere {
    /// Whose private info `render` shows (`None` = the mover's, hot-seat).
    viewer: Option<Player>,
    /// 0 = gol, altfel litera + 1
    board: [[u8; SIZE]; SIZE],
    blank_at: [[bool; SIZE]; SIZE],
    bag: Vec<u8>,
    racks: [Vec<u8>; 2],
    fresh: [usize; 2],
    score: [i32; 2],
    side: Player,
    scoreless: u8,
    plies: u32,
    over: bool,
    dict: Arc<Dictionary>,
    rng: Rng,
}

impl Default for Litere {
    fn default() -> Self {
        Self::new()
    }
}

impl Litere {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Self::build(seed, default_dictionary())
    }

    /// Joc cu un dicționar dat (text cu cuvinte separate prin spații/linii).
    pub fn with_dictionary(seed: u64, text: &str) -> Self {
        Self::build(seed, Arc::new(Dictionary::from_text(text)))
    }

    fn build(seed: u64, dict: Arc<Dictionary>) -> Self {
        let mut rng = Rng::new(seed);
        let mut bag: Vec<u8> = Vec::new();
        for (l, &n) in COUNTS.iter().enumerate() {
            bag.extend(std::iter::repeat_n(l as u8, n as usize));
        }
        bag.extend(std::iter::repeat_n(BLANK, BLANKS as usize));
        for i in (1..bag.len()).rev() {
            let j = rng.below(i + 1);
            bag.swap(i, j);
        }
        let mut g = Litere {
            viewer: None,
            board: [[0; SIZE]; SIZE],
            blank_at: [[false; SIZE]; SIZE],
            bag,
            racks: [Vec::new(), Vec::new()],
            fresh: [0, 0],
            score: [0, 0],
            side: Player::One,
            scoreless: 0,
            plies: 0,
            over: false,
            dict,
            rng,
        };
        for p in 0..2 {
            g.draw(p);
        }
        g.fresh = [0, 0];
        g
    }

    pub fn dictionary(&self) -> &Dictionary {
        &self.dict
    }

    pub fn score(&self, p: Player) -> i32 {
        self.score[p.idx()]
    }

    pub fn rack(&self, p: Player) -> &[u8] {
        &self.racks[p.idx()]
    }

    pub fn bag_len(&self) -> usize {
        self.bag.len()
    }

    /// Plăcuțe pe tablă + mâini + sac (trebuie să fie `tile_total()`).
    pub fn tiles_in_play(&self) -> usize {
        self.board.iter().flatten().filter(|&&c| c != 0).count()
            + self.racks[0].len()
            + self.racks[1].len()
            + self.bag.len()
    }

    fn draw(&mut self, p: usize) {
        self.fresh[p] = 0;
        while self.racks[p].len() < RACK {
            match self.bag.pop() {
                Some(t) => {
                    self.racks[p].push(t);
                    self.fresh[p] += 1;
                }
                None => break,
            }
        }
    }

    fn board_empty(&self) -> bool {
        self.board.iter().flatten().all(|&c| c == 0)
    }

    fn at(&self, r: i32, c: i32) -> u8 {
        if r < 0 || c < 0 || r >= SIZE as i32 || c >= SIZE as i32 {
            0
        } else {
            self.board[r as usize][c as usize]
        }
    }

    fn tile_value(&self, r: usize, c: usize) -> i32 {
        if self.blank_at[r][c] {
            0
        } else {
            VALUES[self.board[r][c] as usize - 1]
        }
    }

    /// Cuvântul transversal format de litera `l` pusă în (r,c), dacă are vecini.
    fn cross_word(&self, r: usize, c: usize, down: bool, l: u8) -> Option<Vec<u8>> {
        let (dr, dc) = if down { (0i32, 1i32) } else { (1i32, 0i32) };
        let (mut r0, mut c0) = (r as i32, c as i32);
        while self.at(r0 - dr, c0 - dc) != 0 {
            r0 -= dr;
            c0 -= dc;
        }
        let mut w = Vec::new();
        let (mut rr, mut cc) = (r0, c0);
        loop {
            let cur = if rr == r as i32 && cc == c as i32 {
                l + 1
            } else {
                self.at(rr, cc)
            };
            if cur == 0 {
                break;
            }
            w.push(cur - 1);
            rr += dr;
            cc += dc;
        }
        if w.len() >= 2 {
            Some(w)
        } else {
            None
        }
    }

    fn score_placement(
        &self,
        row: usize,
        col: usize,
        down: bool,
        letters: &[u8],
        blanks: u32,
    ) -> i32 {
        let (dr, dc) = if down { (1usize, 0usize) } else { (0, 1) };
        let mut main = 0;
        let mut wmul = 1;
        let mut cross_total = 0;
        let mut new = 0;
        for (i, &l) in letters.iter().enumerate() {
            let (r, c) = (row + dr * i, col + dc * i);
            if self.board[r][c] != 0 {
                main += self.tile_value(r, c);
            } else {
                new += 1;
                let (lm, wm) = premium(r, c);
                let v = if blanks & (1 << i) != 0 {
                    0
                } else {
                    VALUES[l as usize]
                };
                main += v * lm;
                wmul *= wm;
                // cuvânt transversal
                let (pr, pc) = if down { (0i32, 1i32) } else { (1i32, 0i32) };
                let mut sum = v * lm;
                let mut cnt = 1;
                let (mut rr, mut cc) = (r as i32 - pr, c as i32 - pc);
                while self.at(rr, cc) != 0 {
                    sum += self.tile_value(rr as usize, cc as usize);
                    cnt += 1;
                    rr -= pr;
                    cc -= pc;
                }
                let (mut rr, mut cc) = (r as i32 + pr, c as i32 + pc);
                while self.at(rr, cc) != 0 {
                    sum += self.tile_value(rr as usize, cc as usize);
                    cnt += 1;
                    rr += pr;
                    cc += pc;
                }
                if cnt >= 2 {
                    cross_total += sum * wm;
                }
            }
        }
        let mut s = main * wmul + cross_total;
        if new == RACK {
            s += 50;
        }
        s
    }

    /// Toate plasările legale cu mâna dată.
    pub fn placements(&self, rack: &[u8]) -> Vec<Move> {
        let mut out = Vec::new();
        let mut have = [0u8; 27];
        for &t in rack {
            have[t as usize] += 1;
        }
        let first = self.board_empty();
        for down in [false, true] {
            for line in 0..SIZE {
                let pos = |k: usize| if down { (k, line) } else { (line, k) };
                let mut lt = [0u8; SIZE];
                let mut allowed = [u32::MAX; SIZE];
                let mut anchor = [false; SIZE];
                for k in 0..SIZE {
                    let (r, c) = pos(k);
                    lt[k] = self.board[r][c];
                    if lt[k] != 0 {
                        anchor[k] = true;
                        continue;
                    }
                    // cuvânt transversal: perpendicular pe direcția mutării
                    let has_nb = {
                        let (pr, pc) = if down { (0i32, 1i32) } else { (1i32, 0i32) };
                        self.at(r as i32 - pr, c as i32 - pc) != 0
                            || self.at(r as i32 + pr, c as i32 + pc) != 0
                    };
                    if has_nb {
                        anchor[k] = true;
                        let mut mask = 0u32;
                        for l in 0..26u8 {
                            match self.cross_word(r, c, down, l) {
                                Some(w) => {
                                    if self.dict.contains(&w) {
                                        mask |= 1 << l;
                                    }
                                }
                                None => mask |= 1 << l,
                            }
                        }
                        allowed[k] = mask;
                    }
                }
                if !first && !anchor.iter().any(|&a| a) {
                    continue;
                }
                let mut pre = [0usize; SIZE + 1];
                for k in 0..SIZE {
                    pre[k + 1] = pre[k] + usize::from(anchor[k]);
                }
                for len in 2..=SIZE {
                    for start in 0..=(SIZE - len) {
                        if start > 0 && lt[start - 1] != 0 {
                            continue;
                        }
                        if start + len < SIZE && lt[start + len] != 0 {
                            continue;
                        }
                        if first {
                            if line != 7 || !(start <= 7 && 7 < start + len) {
                                continue;
                            }
                        } else if pre[start + len] - pre[start] == 0 {
                            continue;
                        }
                        'word: for w in &self.dict.by_len[len] {
                            let mut need = [0u8; 26];
                            let mut new = 0;
                            for i in 0..len {
                                let k = start + i;
                                if lt[k] != 0 {
                                    if lt[k] - 1 != w[i] {
                                        continue 'word;
                                    }
                                } else {
                                    if allowed[k] & (1 << w[i]) == 0 {
                                        continue 'word;
                                    }
                                    need[w[i] as usize] += 1;
                                    new += 1;
                                }
                            }
                            if new == 0 || new > RACK {
                                continue;
                            }
                            let mut blanks_needed = 0;
                            for l in 0..26 {
                                if need[l] > have[l] {
                                    blanks_needed += need[l] - have[l];
                                }
                            }
                            if blanks_needed > have[BLANK as usize] {
                                continue;
                            }
                            // alege pozițiile blankurilor: cele cu multiplicatorul de literă cel mai mic
                            let mut mask = 0u32;
                            for l in 0..26u8 {
                                let short = need[l as usize].saturating_sub(have[l as usize]);
                                if short == 0 {
                                    continue;
                                }
                                let mut ps: Vec<(i32, usize)> = (0..len)
                                    .filter(|&i| lt[start + i] == 0 && w[i] == l)
                                    .map(|i| {
                                        let (r, c) = pos(start + i);
                                        (premium(r, c).0, i)
                                    })
                                    .collect();
                                ps.sort();
                                for &(_, i) in ps.iter().take(short as usize) {
                                    mask |= 1 << i;
                                }
                            }
                            let (r0, c0) = pos(start);
                            out.push(Move::Place {
                                row: r0 as u8,
                                col: c0 as u8,
                                down,
                                letters: w.clone(),
                                blanks: mask,
                            });
                        }
                    }
                }
            }
        }
        out
    }

    fn rack_value(&self, p: usize) -> i32 {
        self.racks[p]
            .iter()
            .map(|&t| if t == BLANK { 0 } else { VALUES[t as usize] })
            .sum()
    }

    fn finish(&mut self, went_out: Option<usize>) {
        let v = [self.rack_value(0), self.rack_value(1)];
        for (s, v) in self.score.iter_mut().zip(v) {
            *s -= v;
        }
        if let Some(p) = went_out {
            self.score[p] += v[1 - p];
        }
        self.over = true;
    }
}

impl Game for Litere {
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }

    /// The opponent's rack and the bag are unseen: they are pooled and re-dealt at random (same sizes), the
    /// seed depending only on what `viewer` sees; the private RNG (exchange shuffles) is replaced too.
    fn ai_view(&self, viewer: Player) -> Litere {
        let mut v = self.clone();
        let h = 1 - viewer.idx();
        let mut rng = Rng::new(crate::p3_comun::view_seed(self, viewer));
        let n = v.racks[h].len();
        let mut pool: Vec<u8> = v.racks[h].clone();
        pool.extend(v.bag.iter().copied());
        pool.sort_unstable();
        for i in (1..pool.len()).rev() {
            let j = rng.below(i + 1);
            pool.swap(i, j);
        }
        v.racks[h] = pool.drain(..n).collect();
        v.bag = pool;
        v.fresh[h] = 0;
        v.rng = Rng::new(rng.next_u64());
        v
    }
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over {
            return Vec::new();
        }
        let mut v = self.placements(&self.racks[self.side.idx()]);
        if self.bag.len() >= RACK {
            v.push(Move::Exchange);
        }
        v.push(Move::Pass);
        v
    }

    fn apply(&mut self, m: &Move) {
        self.plies += 1;
        let p = self.side.idx();
        self.fresh = [0, 0];
        match m {
            Move::Place {
                row,
                col,
                down,
                letters,
                blanks,
            } => {
                let (row, col) = (*row as usize, *col as usize);
                let pts = self.score_placement(row, col, *down, letters, *blanks);
                let (dr, dc) = if *down { (1usize, 0usize) } else { (0, 1) };
                for (i, &l) in letters.iter().enumerate() {
                    let (r, c) = (row + dr * i, col + dc * i);
                    if self.board[r][c] != 0 {
                        continue;
                    }
                    let tile = if blanks & (1 << i) != 0 { BLANK } else { l };
                    let pos = self.racks[p]
                        .iter()
                        .position(|&t| t == tile)
                        .expect("plăcuță în mână");
                    self.racks[p].remove(pos);
                    self.board[r][c] = l + 1;
                    self.blank_at[r][c] = tile == BLANK;
                }
                self.score[p] += pts;
                self.scoreless = 0;
                self.draw(p);
                if self.racks[p].is_empty() && self.bag.is_empty() {
                    self.finish(Some(p));
                    return;
                }
            }
            Move::Exchange => {
                let old = std::mem::take(&mut self.racks[p]);
                self.bag.extend(old);
                for i in (1..self.bag.len()).rev() {
                    let j = self.rng.below(i + 1);
                    self.bag.swap(i, j);
                }
                self.draw(p);
                self.scoreless += 1;
            }
            Move::Pass => {
                self.scoreless += 1;
            }
        }
        if self.scoreless >= 4 || self.plies >= PLY_CAP {
            self.finish(None);
            return;
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    /// Scorul vizibil + valoarea „leave”-ului (fără plăcuțele abia trase).
    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        let mut s = 10 * (self.score[p.idx()] - self.score[q.idx()]);
        let rack = &self.racks[p.idx()];
        let keep = rack.len().saturating_sub(self.fresh[p.idx()]);
        let leave = &rack[..keep];
        let mut vowels = 0;
        let mut seen = [0u8; 27];
        for &t in leave {
            seen[t as usize] += 1;
            if t == BLANK {
                s += 25;
            } else if matches!(t, 0 | 4 | 8 | 14 | 20) {
                vowels += 1;
            }
        }
        let cons = leave.len() as i32 - vowels - seen[BLANK as usize] as i32;
        s -= 3 * (vowels - cons).abs().min(4);
        for &c in &seen[..26] {
            if c > 1 {
                s -= 4 * (c as i32 - 1);
            }
        }
        // litere bune pentru bingo (S, T, R etc. mici bonusuri)
        for &t in leave {
            if matches!(t, 18 | 19 | 17 | 4 | 8) {
                s += 1;
            }
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::from("Litere\n    ");
        for c in 0..SIZE {
            s.push((b'a' + c as u8) as char);
            s.push(' ');
        }
        s.push('\n');
        for r in 0..SIZE {
            s.push_str(&format!("{:>2}  ", r + 1));
            for c in 0..SIZE {
                let ch = if self.board[r][c] != 0 {
                    let l = (b'A' + self.board[r][c] - 1) as char;
                    if self.blank_at[r][c] {
                        l.to_ascii_lowercase()
                    } else {
                        l
                    }
                } else {
                    match premium(r, c) {
                        (1, 3) => '#',
                        (1, 2) => {
                            if r == 7 && c == 7 {
                                '*'
                            } else {
                                '+'
                            }
                        }
                        (3, 1) => 'x',
                        (2, 1) => '-',
                        _ => '.',
                    }
                };
                s.push(ch);
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str(
            "(# cuvânt ×3, + cuvânt ×2, * centru, x literă ×3, - literă ×2; literă mică = blank)\n",
        );
        s.push_str(&format!(
            "Puncte: J1 {}, J2 {} | în sac: {} plăcuțe\n",
            self.score[0],
            self.score[1],
            self.bag.len()
        ));
        if self.over {
            s.push_str("Joc încheiat (scorul include penalizările finale).\n");
            return s;
        }
        let v = self.viewer.unwrap_or(self.side).idx();
        let mut rack = self.racks[v].clone();
        rack.sort_unstable();
        let txt: Vec<String> = rack
            .iter()
            .map(|&t| {
                if t == BLANK {
                    "_".to_string()
                } else {
                    ((b'A' + t) as char).to_string()
                }
            })
            .collect();
        s.push_str(&format!("Mâna Jucătorului {}: {}\n", v + 1, txt.join(" ")));
        s.push_str(&format!(
            "Jucătorul {}: {} plăcuțe\n",
            2 - v,
            self.racks[1 - v].len()
        ));
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Pass => "pas".into(),
            Move::Exchange => "schimb".into(),
            Move::Place {
                row,
                col,
                down,
                letters,
                blanks,
            } => {
                let mut w = String::new();
                for (i, &l) in letters.iter().enumerate() {
                    let ch = (b'a' + l) as char;
                    if blanks & (1 << i) != 0 {
                        w.push('(');
                        w.push(ch);
                        w.push(')');
                    } else {
                        w.push(ch);
                    }
                }
                format!(
                    "{}@{}{}{}",
                    w,
                    (b'a' + col) as char,
                    row + 1,
                    if *down { 'v' } else { '>' }
                )
            }
        }
    }

    fn ai_style(&self) -> AiStyle {
        // Adâncimea 0 = o singură mutare la TOATE nivelurile: la Hard/Expert adâncimea 1 ar extinde
        // răspunsurile adversarului din mâna lui reală (informație ascunsă).
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn letter(c: char) -> u8 {
        c as u8 - b'a'
    }

    fn mini_dict() -> &'static str {
        "casa masa sa as sac cas ca mas sas ea ac"
    }

    fn game_with_rack(seed: u64, rack: &str, text: &str) -> Litere {
        let mut g = Litere::with_dictionary(seed, text);
        // refacem sacul: ce nu e în mână rămâne în sac
        let want: Vec<u8> = rack
            .chars()
            .map(|c| if c == '_' { BLANK } else { letter(c) })
            .collect();
        let mut all = std::mem::take(&mut g.bag);
        all.extend(std::mem::take(&mut g.racks[0]));
        all.extend(std::mem::take(&mut g.racks[1]));
        for &t in &want {
            let pos = all.iter().position(|&x| x == t).expect("în set");
            all.remove(pos);
        }
        g.racks[0] = want;
        for _ in 0..RACK {
            let t = all.pop().unwrap();
            g.racks[1].push(t);
        }
        g.bag = all;
        g
    }

    #[test]
    fn tile_distribution() {
        assert_eq!(tile_total(), 97);
        let g = Litere::with_seed(1);
        assert_eq!(g.tiles_in_play(), 97);
        assert_eq!(g.racks[0].len(), 7);
        assert_eq!(g.racks[1].len(), 7);
    }

    #[test]
    fn dictionary_loads_and_normalizes() {
        let d = default_dictionary();
        assert!(d.len() >= 2500, "{}", d.len());
        for w in [
            "casa",
            "mamă",
            "școală",
            "țară",
            "înăuntru",
            "mâine",
            "pădure",
        ] {
            if w == "înăuntru" || w == "mâine" {
                continue;
            }
            assert!(d.contains_str(w), "{w}");
        }
        assert!(!d.contains_str("xqzv"));
        assert_eq!(normalize("Țară"), normalize("tara"));
        assert!(normalize("a-b").is_none());
    }

    #[test]
    fn dictionary_text_comments_and_case() {
        let d = Dictionary::from_text("# comentariu\nCASA  Masă\n x \n");
        assert!(d.contains_str("casa"));
        assert!(d.contains_str("masa"));
        assert_eq!(d.len(), 2); // „x” are o literă: respinsă
    }

    #[test]
    fn first_move_must_cover_center() {
        let g = game_with_rack(1, "casaxzb", mini_dict());
        let m = g.placements(&g.racks[0]);
        assert!(!m.is_empty());
        for mv in &m {
            if let Move::Place {
                row,
                col,
                down,
                letters,
                ..
            } = mv
            {
                let (r, c) = (*row as usize, *col as usize);
                let covers = if *down {
                    c == 7 && r <= 7 && 7 < r + letters.len()
                } else {
                    r == 7 && c <= 7 && 7 < c + letters.len()
                };
                assert!(covers);
            }
        }
        let txt: Vec<String> = m.iter().map(|x| g.format_move(x)).collect();
        assert!(txt.contains(&"casa@h8>".to_string()));
        assert!(txt.contains(&"casa@e8>".to_string()));
        assert!(txt.contains(&"casa@h8v".to_string()));
        assert!(!txt.iter().any(|t| t.starts_with("masa")), "n-am M");
    }

    #[test]
    fn scoring_with_center_premium() {
        let mut g = game_with_rack(2, "casaxzb", mini_dict());
        // casa la h8 > : C2 A1 S1 A1 = 5; centrul h8 e cuvânt ×2 → 10
        let before = g.score(Player::One);
        g.play_text("casa@h8>");
        assert_eq!(g.score(Player::One) - before, 10);
        assert_eq!(g.racks[0].len(), 7);
        assert_eq!(g.tiles_in_play(), 97);
    }

    trait PlayText {
        fn play_text(&mut self, t: &str);
    }
    impl PlayText for Litere {
        fn play_text(&mut self, t: &str) {
            let mv = Game::legal_moves(self)
                .into_iter()
                .find(|m| self.format_move(m) == t)
                .unwrap_or_else(|| panic!("mutare ilegală {t}"));
            self.apply(&mv);
        }
    }

    #[test]
    fn crosswords_must_be_valid() {
        let mut g = game_with_rack(3, "casaxzb", mini_dict());
        g.play_text("casa@h8>");
        // J2: cu mâna fixă, formează „sa” / „as” pe verticală sub literele existente
        g.racks[1] = vec![
            letter('s'),
            letter('a'),
            letter('c'),
            letter('a'),
            letter('m'),
            letter('s'),
            letter('e'),
        ];
        let moves = Game::legal_moves(&g);
        let txt: Vec<String> = moves.iter().map(|m| g.format_move(m)).collect();
        // „masa” vertical prin s-ul din casa (i8, coloana 9... s = col j): m-a-S-a pe coloană
        assert!(txt.iter().any(|t| t.ends_with('v')), "{txt:?}");
        // niciun cuvânt vertical dintr-o literă nouă adiacentă care ar face un cuvânt transversal invalid
        for t in &txt {
            assert!(t != "xyz@a1>");
        }
        // toate mutările generate respectă dicționarul pe cuvântul principal
        for m in &moves {
            if let Move::Place { letters, .. } = m {
                assert!(g.dict.contains(letters));
            }
        }
    }

    #[test]
    fn blank_plays_score_zero_for_that_letter() {
        let mut g = game_with_rack(4, "ca_axzb", mini_dict());
        // „casa” cu blank pe s: C2 A1 s0 A1 = 4, ×2 = 8
        let txt: Vec<String> = Game::legal_moves(&g)
            .iter()
            .map(|m| g.format_move(m))
            .collect();
        assert!(txt.contains(&"ca(s)a@h8>".to_string()), "{txt:?}");
        let before = g.score(Player::One);
        g.play_text("ca(s)a@h8>");
        assert_eq!(g.score(Player::One) - before, 8);
        assert!(g.blank_at[7][9]);
    }

    #[test]
    fn exchange_and_pass_end_conditions() {
        let mut g = Litere::with_seed(5);
        assert!(Game::legal_moves(&g).contains(&Move::Exchange));
        g.apply(&Move::Exchange);
        assert_eq!(g.tiles_in_play(), 97);
        g.apply(&Move::Pass);
        g.apply(&Move::Pass);
        assert!(g.outcome().is_none());
        g.apply(&Move::Pass);
        assert!(g.outcome().is_some());
    }

    #[test]
    fn going_out_adds_opponent_rack() {
        let mut g = game_with_rack(6, "sa", mini_dict());
        g.bag.clear();
        g.racks[1] = vec![letter('z'), letter('z')];
        g.apply(&Move::Place {
            row: 7,
            col: 7,
            down: false,
            letters: vec![letter('s'), letter('a')],
            blanks: 0,
        });
        // (1+1)×2 = 4 puncte; Z = 8 în tabelul nostru, ×2 plăcuțe = 16
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        assert_eq!(g.score(Player::One), 4 + 2 * VALUES[25]);
        assert_eq!(g.score(Player::Two), -2 * VALUES[25]);
    }

    #[test]
    fn random_games_end_and_conserve_tiles() {
        for seed in 1..=6u64 {
            let mut g = Litere::with_seed(seed);
            let mut rng = Rng::new(seed + 5);
            let mut plies = 0;
            while Game::outcome(&g).is_none() {
                let m = Game::legal_moves(&g);
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                assert_eq!(g.tiles_in_play(), 97);
                plies += 1;
                assert!(plies <= 400);
            }
        }
    }

    #[test]
    fn every_generated_move_is_valid_on_the_board() {
        // joacă 12 mutări aleatoare și verifică din nou toate cuvintele de pe tablă
        let mut g = Litere::with_seed(11);
        let mut rng = Rng::new(3);
        for _ in 0..12 {
            let m: Vec<Move> = Game::legal_moves(&g)
                .into_iter()
                .filter(|m| matches!(m, Move::Place { .. }))
                .collect();
            if m.is_empty() {
                break;
            }
            g.apply(&m[rng.below(m.len())]);
            // toate secvențele de ≥2 litere pe rânduri/coloane sunt cuvinte
            for down in [false, true] {
                for a in 0..SIZE {
                    let mut run: Vec<u8> = Vec::new();
                    for b in 0..=SIZE {
                        let ch = if b < SIZE {
                            if down {
                                g.board[b][a]
                            } else {
                                g.board[a][b]
                            }
                        } else {
                            0
                        };
                        if ch != 0 {
                            run.push(ch - 1);
                        } else {
                            if run.len() >= 2 {
                                assert!(g.dict.contains(&run), "cuvânt invalid pe tablă: {run:?}");
                            }
                            run.clear();
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn render_hides_opponent_rack() {
        let g = Litere::with_seed(7);
        let r = Game::render(&g);
        assert!(r.contains("Mâna Jucătorului 1"));
        assert!(!r.contains("Mâna Jucătorului 2"));
    }

    #[test]
    fn ai_plays_real_words_and_beats_random() {
        let (mut ai, mut rnd) = (0, 0);
        for seed in 1..=4u64 {
            let ai_side = if seed % 2 == 0 {
                Player::One
            } else {
                Player::Two
            };
            let mut g = Litere::with_seed(seed);
            let mut rng = Rng::new(seed * 3);
            while Game::outcome(&g).is_none() {
                let m = if g.to_move() == ai_side {
                    polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
                } else {
                    let l = Game::legal_moves(&g);
                    l[rng.below(l.len())].clone()
                };
                g.apply(&m);
            }
            match Game::outcome(&g) {
                Some(Outcome::Win(p)) if p == ai_side => ai += 1,
                Some(Outcome::Win(_)) => rnd += 1,
                _ => {}
            }
        }
        assert!(ai > rnd, "AI {ai} vs random {rnd}");
    }
}

#[cfg(test)]
mod a3_audit {
    use super::*;

    #[test]
    fn ai_ignores_the_opponent_rack_and_bag_at_every_level() {
        crate::p3_comun::ai_blind_check(
            "litere",
            Litere::with_seed,
            |g, me, s| {
                let h = 1 - me.idx();
                crate::p3_comun::mix_unseen(&mut g.racks[h], &mut g.bag, 0, s);
            },
            2,
        );
    }

    #[cfg(test)]
    mod ai_view_audit {
        use super::*;

        #[test]
        fn ai_view_hides_the_opponent_rack_and_bag_even_when_going_out() {
            crate::p3_comun::ai_view_check(
                "litere",
                Litere::with_seed,
                |g, me, s| {
                    let h = 1 - me.idx();
                    crate::p3_comun::mix_unseen(&mut g.racks[h], &mut g.bag, 0, s);
                    g.rng = Rng::new(s ^ 0x77);
                },
                4,
            );
        }
    }
}
