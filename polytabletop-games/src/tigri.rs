//! Tigri și capre (Nepal; „Bagh-Chal”): tablă de 5×5 puncte, linii ortogonale plus
//! diagonale prin punctele „pare” (rând + coloană par). Patru tigri (Jucătorul 2) pornesc
//! în colțuri, 20 de capre (Jucătorul 1, mută primul) sunt în mână. Caprele se plasează
//! pe rând, câte una pe orice punct liber, apoi (după ce au fost plasate toate) se mută
//! cu un pas pe un punct vecin liber. Tigrii se mută cu un pas sau sar peste o capră
//! vecină, pe aceeași linie, pe punctul liber de după ea, capturând-o (captura nu e
//! obligatorie). Tigrii câștigă cu 5 capre capturate; caprele câștigă dacă toți tigrii
//! sunt imobilizați. Remiză după 100 de semi-mutări fără captură în faza de mutare
//! (regulă proprie, în loc de repetiția pozițiilor).
//!
//! Verificare surse: en.wikipedia.org/wiki/Bagh-chal (tabla, 4+20 piese, 5 capturi,
//! imobilizarea tigrilor). Implementare proprie.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const S: usize = 5;
const N: usize = S * S;
const GOATS: u8 = 20;
const WIN_CAPTURES: u8 = 5;
const QUIET_LIMIT: u16 = 100;

const DIRS: [(i32, i32); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

fn name(i: usize) -> String {
    polytabletop_core::square_name(i % S, i / S)
}

fn step(i: usize, d: usize) -> Option<usize> {
    let (r, c) = ((i / S) as i32, (i % S) as i32);
    let (dr, dc) = DIRS[d];
    if dr != 0 && dc != 0 && (r + c) % 2 != 0 {
        return None;
    }
    let (nr, nc) = (r + dr, c + dc);
    if nr < 0 || nc < 0 || nr >= S as i32 || nc >= S as i32 {
        None
    } else {
        Some(nr as usize * S + nc as usize)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    /// `None` = plasarea unei capre din mână.
    pub from: Option<u8>,
    pub to: u8,
    /// Capra capturată (doar la sărituri ale tigrilor).
    pub take: Option<u8>,
}

#[derive(Clone)]
pub struct Tigri {
    /// 0 gol, 1 capră, 2 tigru.
    b: [u8; N],
    hand: u8,
    taken: u8,
    side: Player,
    quiet: u16,
}

impl Default for Tigri {
    fn default() -> Self {
        Self::new()
    }
}

impl Tigri {
    pub fn new() -> Self {
        let mut b = [0u8; N];
        for i in [0, S - 1, N - S, N - 1] {
            b[i] = 2;
        }
        Tigri {
            b,
            hand: GOATS,
            taken: 0,
            side: Player::One,
            quiet: 0,
        }
    }

    fn tiger_moves(&self, from: usize, out: &mut Vec<Mv>) {
        for d in 0..8 {
            if let Some(t) = step(from, d) {
                if self.b[t] == 0 {
                    out.push(Mv {
                        from: Some(from as u8),
                        to: t as u8,
                        take: None,
                    });
                } else if self.b[t] == 1 {
                    if let Some(j) = step(t, d) {
                        if self.b[j] == 0 {
                            out.push(Mv {
                                from: Some(from as u8),
                                to: j as u8,
                                take: Some(t as u8),
                            });
                        }
                    }
                }
            }
        }
    }

    fn gen(&self) -> Vec<Mv> {
        let mut out = Vec::new();
        if self.side == Player::One {
            if self.hand > 0 {
                for i in 0..N {
                    if self.b[i] == 0 {
                        out.push(Mv {
                            from: None,
                            to: i as u8,
                            take: None,
                        });
                    }
                }
            } else {
                for i in 0..N {
                    if self.b[i] != 1 {
                        continue;
                    }
                    for d in 0..8 {
                        if let Some(t) = step(i, d) {
                            if self.b[t] == 0 {
                                out.push(Mv {
                                    from: Some(i as u8),
                                    to: t as u8,
                                    take: None,
                                });
                            }
                        }
                    }
                }
            }
        } else {
            for i in 0..N {
                if self.b[i] == 2 {
                    self.tiger_moves(i, &mut out);
                }
            }
        }
        out
    }

    fn tigers_trapped(&self) -> bool {
        (0..N).all(|i| {
            if self.b[i] != 2 {
                return true;
            }
            let mut v = Vec::new();
            self.tiger_moves(i, &mut v);
            v.is_empty()
        })
    }
}

impl Game for Tigri {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        self.gen()
    }

    fn apply(&mut self, m: &Mv) {
        let me = self.side.idx() as u8 + 1;
        match m.from {
            Some(f) => self.b[f as usize] = 0,
            None => self.hand -= 1,
        }
        self.b[m.to as usize] = me;
        if let Some(t) = m.take {
            self.b[t as usize] = 0;
            self.taken += 1;
            self.quiet = 0;
        } else if self.hand == 0 {
            self.quiet += 1;
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.taken >= WIN_CAPTURES {
            return Some(Outcome::Win(Player::Two));
        }
        if self.tigers_trapped() {
            return Some(Outcome::Win(Player::One));
        }
        if self.quiet >= QUIET_LIMIT {
            return Some(Outcome::Draw);
        }
        if self.gen().is_empty() {
            // Caprele fără nicio mutare pierd (rar); tigrii imobilizați sunt tratați mai sus.
            return Some(Outcome::Win(self.side.other()));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        // Pozitiv = bine pentru capre (Jucătorul 1).
        let mut exposed = 0;
        let mut stuck = 0;
        let mut mob = 0;
        for i in 0..N {
            if self.b[i] != 2 {
                continue;
            }
            let mut v = Vec::new();
            self.tiger_moves(i, &mut v);
            if v.is_empty() {
                stuck += 1;
            }
            mob += v.len() as i32;
            exposed += v.iter().filter(|m| m.take.is_some()).count() as i32;
        }
        let left = GOATS as i32 - self.taken as i32;
        let mut v = -300 * self.taken as i32 + 8 * left - 12 * mob + 70 * stuck - 45 * exposed;
        // Tigrii la mutare pot executa imediat o captură: pericol mai mare.
        if self.side == Player::Two && exposed > 0 {
            v -= 150;
        }
        if p == Player::One {
            v
        } else {
            -v
        }
    }

    fn render(&self) -> String {
        let mut s = String::from("   a   b   c   d   e\n");
        for r in (0..S).rev() {
            s.push_str(&format!("{}  ", r + 1));
            for c in 0..S {
                s.push(match self.b[r * S + c] {
                    1 => 'G',
                    2 => 'T',
                    _ => '+',
                });
                if c + 1 < S {
                    s.push_str(" - ");
                }
            }
            s.push('\n');
            if r > 0 {
                s.push_str("   ");
                for c in 0..S {
                    s.push('|');
                    if c + 1 < S {
                        s.push(' ');
                        s.push(if (r - 1 + c) % 2 == 0 { '/' } else { '\\' });
                        s.push(' ');
                    }
                }
                s.push('\n');
            }
        }
        s.push_str(&format!(
            "  G = capre (Jucătorul 1), T = tigri (Jucătorul 2). În mână: {}  Capturate: {}/{}\n",
            self.hand, self.taken, WIN_CAPTURES
        ));
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        match (m.from, m.take) {
            (None, _) => name(m.to as usize),
            (Some(f), None) => format!("{}-{}", name(f as usize), name(m.to as usize)),
            (Some(f), Some(_)) => format!("{}x{}", name(f as usize), name(m.to as usize)),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 500,
            max_depth: 10,
        }
    }

    fn hash_key(&self) -> Option<u64> {
        let mut h = polytabletop_core::hash_mix(
            1 << 40
                | (self.side.idx() as u64) << 32
                | (self.quiet as u64)
                | (self.hand as u64) << 8
                | (self.taken as u64) << 20,
        );
        for (i, &v) in self.b.iter().enumerate() {
            if v != 0 {
                h ^= polytabletop_core::hash_mix((v as u64) << 16 | i as u64);
            }
        }
        Some(h)
    }

    fn move_score(&self, m: &Mv) -> i32 {
        if m.take.is_some() {
            10
        } else if m.from.is_none() {
            1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    fn at(s: &str) -> usize {
        let b = s.as_bytes();
        (b[1] - b'1') as usize * S + (b[0] - b'a') as usize
    }

    fn texts(g: &Tigri) -> Vec<String> {
        g.legal_moves().iter().map(|m| g.format_move(m)).collect()
    }

    fn play(g: &mut Tigri, s: &str) {
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == s)
            .unwrap_or_else(|| panic!("illegal {s}: {:?}", texts(g)));
        g.apply(&m);
    }

    #[test]
    fn start() {
        let g = Tigri::new();
        assert_eq!(g.b[at("a1")], 2);
        assert_eq!(g.b[at("e5")], 2);
        assert_eq!(g.legal_moves().len(), 21);
        assert_eq!(g.hand, 20);
    }

    #[test]
    fn tiger_jump_captures_goat() {
        let mut g = Tigri::new();
        g.hand = 0;
        g.side = Player::Two;
        g.b[at("b1")] = 1;
        let t = texts(&g);
        assert!(t.contains(&"a1xc1".to_string()), "{t:?}");
        play(&mut g, "a1xc1");
        assert_eq!(g.b[at("b1")], 0);
        assert_eq!(g.b[at("c1")], 2);
        assert_eq!(g.taken, 1);
    }

    #[test]
    fn capture_is_not_compulsory_and_needs_empty_landing() {
        let mut g = Tigri::new();
        g.hand = 0;
        g.side = Player::Two;
        g.b[at("b1")] = 1;
        let t = texts(&g);
        assert!(t.contains(&"a5-a4".to_string()));
        g.b[at("c1")] = 1; // landing occupied
        assert!(!texts(&g).contains(&"a1xc1".to_string()));
    }

    #[test]
    fn diagonals_only_on_even_points() {
        let mut g = Tigri::new();
        g.hand = 0;
        g.side = Player::Two;
        g.b = [0; N];
        g.b[at("b1")] = 2; // odd point: no diagonals
        g.b[at("a1")] = 2; // even: has them
        let t = texts(&g);
        assert!(t.contains(&"a1-b2".to_string()));
        assert!(!t.contains(&"b1-a2".to_string()));
        assert!(!t.contains(&"b1-c2".to_string()));
    }

    #[test]
    fn goats_place_then_move() {
        let mut g = Tigri::new();
        g.hand = 1;
        g.b[at("c3")] = 1;
        play(&mut g, "b1");
        assert_eq!(g.hand, 0);
        g.side = Player::One;
        let t = texts(&g);
        assert!(t.contains(&"c3-c4".to_string()), "{t:?}");
        assert!(!t.iter().any(|s| s.len() == 2));
    }

    #[test]
    fn five_captures_win_for_tigers() {
        let mut g = Tigri::new();
        g.hand = 0;
        g.side = Player::Two;
        g.taken = 4;
        g.b[at("b1")] = 1;
        play(&mut g, "a1xc1");
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn trapped_tigers_lose() {
        let mut g = Tigri::new();
        g.b = [0; N];
        g.hand = 0;
        g.side = Player::Two;
        g.b[at("a1")] = 2;
        for s in ["a2", "b1", "b2", "a3", "c1", "c3"] {
            g.b[at(s)] = 1;
        }
        // Neighbours a2, b1, b2 are goats and every landing point behind them is taken.
        assert!(g.tigers_trapped());
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        // Free one landing point: the tiger can jump again.
        g.b[at("a3")] = 0;
        assert!(!g.tigers_trapped());
        assert_eq!(g.outcome(), None);
    }

    #[test]
    fn random_games_terminate() {
        for seed in 1..=10u64 {
            let mut g = Tigri::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 3000 {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }
}
