//! Dama (dame turcești): tablă 8 × 8, 16 piese pe jucător pe rândurile 2 și 3 (respectiv 6 și 7).
//!
//! Reguli folosite (regulile publice ale „Türk daması”):
//! * Piesele obișnuite se mută un pas înainte sau lateral (nu în diagonală, nu înapoi).
//! * Captura e ortogonală: săritură peste o piesă adversă adiacentă (înainte sau lateral)
//!   pe pătratul liber de după ea. Captura e OBLIGATORIE și trebuie aleasă secvența care
//!   ia cele mai multe piese (regula majorității). Piesele capturate se scot abia la
//!   sfârșitul secvenței și nu pot fi sărite de două ori.
//! * Piesa care ajunge pe ultimul rând devine „dama” (rege) la sfârșitul mutării; dama
//!   se mută oricâte pătrate libere ortogonal și captură prin salt peste o piesă
//!   adversă de la orice distanță, aterizând pe orice pătrat liber de după ea.
//! * Câștigă cine capturează toate piesele adversarului sau îl lasă fără mutări.
//!   Remiză după 80 de semi-mutări fără captură și fără mutarea unei piese obișnuite.
//!
//! Notație: pas `a3-a4`; captură `a3xa5xc5` (pătratele de aterizare, în ordine).

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const W: usize = 8;
const N: usize = 64;
const QUIET_LIMIT: u32 = 80;

fn name(i: usize) -> String {
    format!("{}{}", (b'a' + (i % W) as u8) as char, i / W + 1)
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Move {
    /// pătratele succesive (primul = start); lungime ≥ 2
    pub path: Vec<u8>,
    pub captures: u8,
}

#[derive(Clone)]
pub struct Dama {
    /// 0 gol, 1 om J1, 2 dama J1, 3 om J2, 4 dama J2
    b: [u8; N],
    side: Player,
    quiet: u32,
}

impl Default for Dama {
    fn default() -> Self {
        Self::new()
    }
}

fn owner(v: u8) -> Option<Player> {
    match v {
        1 | 2 => Some(Player::One),
        3 | 4 => Some(Player::Two),
        _ => None,
    }
}

fn is_king(v: u8) -> bool {
    v == 2 || v == 4
}

impl Dama {
    pub fn new() -> Self {
        let mut b = [0u8; N];
        for c in 0..W {
            b[W + c] = 1;
            b[2 * W + c] = 1;
            b[5 * W + c] = 3;
            b[6 * W + c] = 3;
        }
        Dama {
            b,
            side: Player::One,
            quiet: 0,
        }
    }

    pub fn pieces(&self, p: Player) -> usize {
        self.b.iter().filter(|&&v| owner(v) == Some(p)).count()
    }

    fn fwd(p: Player) -> i32 {
        match p {
            Player::One => 1,
            Player::Two => -1,
        }
    }

    fn at(r: i32, c: i32) -> Option<usize> {
        if (0..W as i32).contains(&r) && (0..W as i32).contains(&c) {
            Some(r as usize * W + c as usize)
        } else {
            None
        }
    }

    fn dirs_for(&self, v: u8, p: Player) -> Vec<(i32, i32)> {
        if is_king(v) {
            vec![(1, 0), (-1, 0), (0, 1), (0, -1)]
        } else {
            vec![(Self::fwd(p), 0), (0, 1), (0, -1)]
        }
    }

    /// Secvențele de captură pornind din `from` (o piesă a lui `p`).
    fn capture_seqs(&self, from: usize, p: Player) -> Vec<Move> {
        let mut out = Vec::new();
        let mut bd = self.b;
        let v = bd[from];
        bd[from] = 0; // piesa mutată eliberează pătratul de start
        let mut path = vec![from as u8];
        self.dfs(&bd, from, v, p, 0u64, &mut path, &mut out);
        out
    }

    #[allow(clippy::too_many_arguments)]
    fn dfs(
        &self,
        bd: &[u8; N],
        pos: usize,
        v: u8,
        p: Player,
        taken: u64,
        path: &mut Vec<u8>,
        out: &mut Vec<Move>,
    ) {
        let (r, c) = ((pos / W) as i32, (pos % W) as i32);
        let mut extended = false;
        for (dr, dc) in self.dirs_for(v, p) {
            let mut cur = (r + dr, c + dc);
            if is_king(v) {
                // deplasare peste pătrate libere până la prima piesă
                while let Some(k) = Self::at(cur.0, cur.1) {
                    if bd[k] != 0 {
                        break;
                    }
                    cur = (cur.0 + dr, cur.1 + dc);
                }
            }
            let Some(k) = Self::at(cur.0, cur.1) else {
                continue;
            };
            if bd[k] == 0 || owner(bd[k]) == Some(p) || taken & (1u64 << k) != 0 {
                continue;
            }
            // aterizări: pătrate libere imediat după piesa sărită
            let mut land = (cur.0 + dr, cur.1 + dc);
            while let Some(l) = Self::at(land.0, land.1) {
                if bd[l] != 0 {
                    break;
                }
                extended = true;
                path.push(l as u8);
                self.dfs(bd, l, v, p, taken | (1u64 << k), path, out);
                path.pop();
                if !is_king(v) {
                    break;
                }
                land = (land.0 + dr, land.1 + dc);
            }
        }
        if !extended && path.len() > 1 {
            out.push(Move {
                path: path.clone(),
                captures: taken.count_ones() as u8,
            });
        }
    }

    fn simple_moves(&self, from: usize, p: Player) -> Vec<Move> {
        let v = self.b[from];
        let (r, c) = ((from / W) as i32, (from % W) as i32);
        let mut out = Vec::new();
        for (dr, dc) in self.dirs_for(v, p) {
            let mut cur = (r + dr, c + dc);
            while let Some(k) = Self::at(cur.0, cur.1) {
                if self.b[k] != 0 {
                    break;
                }
                out.push(Move {
                    path: vec![from as u8, k as u8],
                    captures: 0,
                });
                if !is_king(v) {
                    break;
                }
                cur = (cur.0 + dr, cur.1 + dc);
            }
        }
        out
    }

    fn all_moves(&self) -> Vec<Move> {
        let p = self.side;
        let mut caps = Vec::new();
        for i in 0..N {
            if owner(self.b[i]) == Some(p) {
                caps.extend(self.capture_seqs(i, p));
            }
        }
        if !caps.is_empty() {
            let best = caps.iter().map(|m| m.captures).max().unwrap_or(0);
            caps.retain(|m| m.captures == best);
            return caps;
        }
        let mut v = Vec::new();
        for i in 0..N {
            if owner(self.b[i]) == Some(p) {
                v.extend(self.simple_moves(i, p));
            }
        }
        v
    }
}

impl Game for Dama {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        self.all_moves()
    }

    fn apply(&mut self, m: &Move) {
        let p = self.side;
        let from = m.path[0] as usize;
        let to = *m.path.last().unwrap() as usize;
        let mut v = self.b[from];
        let was_man = !is_king(v);
        self.b[from] = 0;
        if m.captures > 0 {
            for w in m.path.windows(2) {
                let (a, b) = (w[0] as usize, w[1] as usize);
                let (ar, ac) = ((a / W) as i32, (a % W) as i32);
                let (br, bc) = ((b / W) as i32, (b % W) as i32);
                let (dr, dc) = ((br - ar).signum(), (bc - ac).signum());
                let (mut r, mut c) = (ar + dr, ac + dc);
                while (r, c) != (br, bc) {
                    let k = r as usize * W + c as usize;
                    if self.b[k] != 0 && owner(self.b[k]) != Some(p) {
                        self.b[k] = 0;
                    }
                    r += dr;
                    c += dc;
                }
            }
            self.quiet = 0;
        } else if was_man {
            self.quiet = 0;
        } else {
            self.quiet += 1;
        }
        let last_row = match p {
            Player::One => W - 1,
            Player::Two => 0,
        };
        if was_man && to / W == last_row {
            v = if p == Player::One { 2 } else { 4 };
        }
        self.b[to] = v;
        self.side = p.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        let (a, b) = (self.pieces(Player::One), self.pieces(Player::Two));
        if a == 0 {
            return Some(Outcome::Win(Player::Two));
        }
        if b == 0 {
            return Some(Outcome::Win(Player::One));
        }
        if self.quiet >= QUIET_LIMIT {
            return Some(Outcome::Draw);
        }
        if self.all_moves().is_empty() {
            return Some(Outcome::Win(self.side.other()));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut s = 0i32;
        for i in 0..N {
            let v = self.b[i];
            let Some(o) = owner(v) else { continue };
            let row = (i / W) as i32;
            let adv = if o == Player::One { row } else { 7 - row };
            let val = if is_king(v) { 330 } else { 100 + 4 * adv };
            s += if o == p { val } else { -val };
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::new();
        for r in (0..W).rev() {
            s.push_str(&format!("{} ", r + 1));
            for c in 0..W {
                let ch = match self.b[r * W + c] {
                    1 => 'x',
                    2 => 'X',
                    3 => 'o',
                    4 => 'O',
                    _ => '.',
                };
                s.push(' ');
                s.push(ch);
            }
            s.push('\n');
        }
        s.push_str("   a b c d e f g h\n");
        s.push_str(
            "x/X = J1 (om/dama), o/O = J2; captura e obligatorie, cu cele mai multe piese.\n",
        );
        s
    }

    fn format_move(&self, m: &Move) -> String {
        let sep = if m.captures > 0 { "x" } else { "-" };
        m.path
            .iter()
            .map(|&i| name(i as usize))
            .collect::<Vec<_>>()
            .join(sep)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    fn n(s: &str) -> usize {
        let b = s.as_bytes();
        (b[1] - b'1') as usize * W + (b[0] - b'a') as usize
    }

    fn empty(side: Player) -> Dama {
        Dama {
            b: [0; N],
            side,
            quiet: 0,
        }
    }

    fn texts(g: &Dama) -> Vec<String> {
        Game::legal_moves(g)
            .iter()
            .map(|m| g.format_move(m))
            .collect()
    }

    #[test]
    fn start_position_and_first_moves() {
        let g = Dama::new();
        assert_eq!(g.pieces(Player::One), 16);
        // 8 oameni din rândul 3 avansează un pas (rândul 2 e blocat, lateral nu e loc)
        assert_eq!(Game::legal_moves(&g).len(), 8);
    }

    #[test]
    fn man_moves_forward_and_sideways_not_back() {
        let mut g = empty(Player::One);
        g.b[n("d4")] = 1;
        g.b[n("h8")] = 3;
        let t = texts(&g);
        assert!(t.contains(&"d4-d5".to_string()));
        assert!(t.contains(&"d4-c4".to_string()));
        assert!(t.contains(&"d4-e4".to_string()));
        assert!(!t.contains(&"d4-d3".to_string()));
    }

    #[test]
    fn capture_is_forced_and_majority_wins() {
        let mut g = empty(Player::One);
        g.b[n("a1")] = 1;
        g.b[n("a2")] = 3;
        g.b[n("a4")] = 3; // a1 x a3 x a5 (două capturi)
        g.b[n("b1")] = 1;
        g.b[n("c1")] = 3; // b1 x d1 (o captură)
        g.b[n("h8")] = 3;
        let t = texts(&g);
        assert_eq!(t, vec!["a1xa3xa5".to_string()]);
    }

    #[test]
    fn man_cannot_capture_backwards_but_king_flies() {
        let mut g = empty(Player::One);
        g.b[n("d5")] = 1;
        g.b[n("d4")] = 3; // în spate: neatins
        g.b[n("h8")] = 3;
        assert!(texts(&g).iter().all(|s| !s.contains('x')));
        let mut k = empty(Player::One);
        k.b[n("a1")] = 2;
        k.b[n("a5")] = 3;
        k.b[n("h8")] = 3;
        let t = texts(&k);
        assert!(t.contains(&"a1xa6".to_string()));
        assert!(t.contains(&"a1xa7".to_string()));
        assert!(t.contains(&"a1xa8".to_string()));
        assert!(!t.contains(&"a1xa5".to_string()));
    }

    #[test]
    fn captured_pieces_stay_until_the_end_and_are_not_jumped_twice() {
        let mut g = empty(Player::One);
        g.b[n("a1")] = 2;
        g.b[n("a3")] = 3;
        g.b[n("c5")] = 3;
        g.b[n("c3")] = 3;
        g.b[n("h8")] = 3;
        // dama: a1xa4 (peste a3), apoi de la a4... numărul maxim e verificat să fie coerent
        let ms = Game::legal_moves(&g);
        let best = ms[0].captures;
        assert!(ms.iter().all(|m| m.captures == best));
        let mut h = g.clone();
        h.apply(&ms[0]);
        assert_eq!(h.pieces(Player::Two), 4 - best as usize);
    }

    #[test]
    fn promotion_on_last_row() {
        let mut g = empty(Player::One);
        g.b[n("d7")] = 1;
        g.b[n("a1")] = 3;
        g.apply(
            &Game::legal_moves(&g)
                .into_iter()
                .find(|m| g.format_move(m) == "d7-d8")
                .unwrap(),
        );
        assert_eq!(g.b[n("d8")], 2);
    }

    #[test]
    fn no_moves_loses() {
        let mut g = empty(Player::Two);
        g.b[n("a8")] = 3;
        g.b[n("a7")] = 1;
        g.b[n("b8")] = 1; // blocat: înainte (a7) ocupat de adversar, fără salt posibil (a6 ocupat)
        g.b[n("a6")] = 1;
        g.b[n("c8")] = 1;
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn random_games_terminate() {
        for seed in 1..=15u64 {
            let mut g = Dama::new();
            let mut rng = Rng::new(seed);
            let mut k = 0;
            while g.outcome().is_none() {
                let m = Game::legal_moves(&g);
                assert!(!m.is_empty(), "{}", g.render());
                g.apply(&m[rng.below(m.len())]);
                k += 1;
                assert!(k < 3000);
            }
        }
    }
}
