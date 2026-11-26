//! Bile (împingere pe hexagoane): tablă hexagonală cu 5 celule pe latură (61 de celule),
//! 14 bile pe jucător. Într-o tură muți una, două sau trei bile proprii, vecine pe aceeași
//! linie, cu un pas într-una din cele șase direcții. Mutare „pe lungime": grupul înaintează
//! pe linia lui și poate împinge 1 sau 2 bile adverse dacă are mai multe bile decât ele
//! (2 contra 1, 3 contra 1 sau 2) și locul de dincolo e liber sau în afara tablei; bilele
//! împinse în afară sunt capturate. Mutare „laterală": grupul se deplasează într-o direcție
//! oblică, toate destinațiile trebuind libere. Câștigă cine scoate 6 bile adverse; remiză
//! după 120 de semi-mutări fără captură (regulă proprie, ca să se termine partidele).
//!
//! Verificare surse: regulile publice ale jocurilor de împingere a bilelor pe tablă
//! hexagonală (sumito 2-1, 3-1, 3-2). Implementare proprie; notația de mai jos e originală.
//! Celule: litera rândului (a jos … i sus) + numărul 1-9 al coloanei.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const W: usize = 9;
const NC: usize = W * W;
const QUIET_LIMIT: u16 = 120;
const WIN_CAPS: u8 = 6;

/// ne, e, se, sw, w, nw (în afișare: rândul de sus are r = -4).
const DIRS: [(i8, i8); 6] = [(1, -1), (1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1)];
const DN: [&str; 6] = ["ne", "e", "se", "sw", "w", "nw"];

fn valid(q: i8, r: i8) -> bool {
    q.abs() <= 4 && r.abs() <= 4 && (q + r).abs() <= 4
}
fn idx(q: i8, r: i8) -> usize {
    ((r + 4) as usize) * W + (q + 4) as usize
}
fn qr(i: usize) -> (i8, i8) {
    ((i % W) as i8 - 4, (i / W) as i8 - 4)
}
fn nb(i: usize, d: usize) -> Option<usize> {
    let (q, r) = qr(i);
    let (q, r) = (q + DIRS[d].0, r + DIRS[d].1);
    if valid(q, r) {
        Some(idx(q, r))
    } else {
        None
    }
}
fn name(i: usize) -> String {
    let (q, r) = qr(i);
    format!("{}{}", (b'a' + (4 - r) as u8) as char, q + 5)
}
fn dist(i: usize) -> i8 {
    let (q, r) = qr(i);
    q.abs().max(r.abs()).max((q + r).abs())
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    /// Prima bilă a grupului (după axă).
    pub first: u8,
    /// Axa grupului (0..3: ne, e, se).
    pub ax: u8,
    pub n: u8,
    pub d: u8,
}

#[derive(Clone)]
pub struct Bile {
    b: [u8; NC],
    caps: [u8; 2],
    side: Player,
    quiet: u16,
}

impl Default for Bile {
    fn default() -> Self {
        Self::new()
    }
}

impl Bile {
    pub fn new() -> Self {
        let mut b = [0u8; NC];
        for r in -4i8..=4 {
            for q in -4i8..=4 {
                if !valid(q, r) {
                    continue;
                }
                let i = idx(q, r);
                if r >= 3 || (r == 2 && (-2..=0).contains(&q)) {
                    b[i] = 1;
                } else if r <= -3 || (r == -2 && (0..=2).contains(&q)) {
                    b[i] = 2;
                }
            }
        }
        Bile {
            b,
            caps: [0, 0],
            side: Player::One,
            quiet: 0,
        }
    }

    fn group(&self, m: &Mv) -> Vec<usize> {
        let mut v = vec![m.first as usize];
        for _ in 1..m.n {
            v.push(nb(*v.last().unwrap(), m.ax as usize).unwrap());
        }
        v
    }

    /// Bilele adverse împinse de o mutare pe lungime (celulele lor), dacă e legală.
    fn pushed(&self, g: &[usize], d: usize, me: u8) -> Option<Vec<usize>> {
        let n = g.len();
        // front = capătul grupului dinspre direcția d.
        let a = if n > 1 {
            // direcția axei = dirul de la g[0] la g[1]
            (0..6).find(|&k| nb(g[0], k) == Some(g[1])).unwrap()
        } else {
            d
        };
        let front = if d == a { g[n - 1] } else { g[0] };
        let mut run = Vec::new();
        let mut cur = nb(front, d);
        while let Some(c) = cur {
            if self.b[c] == 0 {
                return Some(run);
            }
            if self.b[c] == me {
                return None;
            }
            run.push(c);
            if run.len() >= n {
                return None;
            }
            cur = nb(c, d);
        }
        // ieșit de pe tablă
        Some(run)
    }

    fn gen(&self) -> Vec<Mv> {
        let me = self.side.idx() as u8 + 1;
        let mut out = Vec::new();
        for s in 0..NC {
            if self.b[s] != me {
                continue;
            }
            for ax in 0..3usize {
                for n in 1..=3u8 {
                    if n == 1 && ax != 0 {
                        continue;
                    }
                    let mut g = vec![s];
                    let mut ok = true;
                    for _ in 1..n {
                        match nb(*g.last().unwrap(), ax) {
                            Some(t) if self.b[t] == me => g.push(t),
                            _ => {
                                ok = false;
                                break;
                            }
                        }
                    }
                    if !ok {
                        continue;
                    }
                    for d in 0..6usize {
                        let inline = n > 1 && (d == ax || d == ax + 3);
                        let legal = if inline {
                            self.pushed(&g, d, me).is_some()
                        } else {
                            g.iter().all(|&c| nb(c, d).is_some_and(|t| self.b[t] == 0))
                        };
                        if legal {
                            out.push(Mv {
                                first: s as u8,
                                ax: ax as u8,
                                n,
                                d: d as u8,
                            });
                        }
                    }
                }
            }
        }
        out
    }

    /// Numărul de bile adverse scoase de mutare.
    fn captures_of(&self, m: &Mv) -> usize {
        let me = self.side.idx() as u8 + 1;
        let g = self.group(m);
        let d = m.d as usize;
        if m.n > 1 && (d == m.ax as usize || d == m.ax as usize + 3) {
            if let Some(run) = self.pushed(&g, d, me) {
                if let Some(&last) = run.last() {
                    if nb(last, d).is_none() {
                        return 1;
                    }
                }
            }
        }
        0
    }
}

impl Game for Bile {
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
        let g = self.group(m);
        let d = m.d as usize;
        let mut moving: Vec<(usize, u8)> = g.iter().map(|&c| (c, me)).collect();
        if m.n > 1 && (d == m.ax as usize || d == m.ax as usize + 3) {
            if let Some(run) = self.pushed(&g, d, me) {
                for c in run {
                    moving.push((c, 3 - me));
                }
            }
        }
        for &(c, _) in &moving {
            self.b[c] = 0;
        }
        let mut cap = false;
        for &(c, who) in &moving {
            match nb(c, d) {
                Some(t) => self.b[t] = who,
                None => {
                    self.caps[self.side.idx()] += 1;
                    cap = true;
                }
            }
        }
        self.quiet = if cap { 0 } else { self.quiet + 1 };
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.caps[0] >= WIN_CAPS {
            return Some(Outcome::Win(Player::One));
        }
        if self.caps[1] >= WIN_CAPS {
            return Some(Outcome::Win(Player::Two));
        }
        if self.quiet >= QUIET_LIMIT {
            return Some(Outcome::Draw);
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx() as u8 + 1;
        let mut v = 1000 * (self.caps[p.idx()] as i32 - self.caps[p.other().idx()] as i32);
        for i in 0..NC {
            let c = self.b[i];
            if c == 0 {
                continue;
            }
            let sign = if c == me { 1 } else { -1 };
            v += sign * (4 - dist(i) as i32) * 6;
            let mut contact = 0;
            for d in 0..6 {
                if let Some(t) = nb(i, d) {
                    if self.b[t] == c {
                        v += sign * 3;
                    } else if self.b[t] != 0 {
                        contact += 1;
                    }
                }
            }
            if dist(i) == 4 && contact > 0 {
                // bilă de margine, în contact cu adversarul: vulnerabilă
                v -= sign * 30;
            }
        }
        v
    }

    fn render(&self) -> String {
        let mut s = String::new();
        for r in -4i8..=4 {
            let lo = (-4).max(-4 - r);
            let hi = 4.min(4 - r);
            s.push_str(&format!("{} ", (b'a' + (4 - r) as u8) as char));
            s.push_str(&" ".repeat(r.unsigned_abs() as usize));
            for q in lo..=hi {
                s.push(match self.b[idx(q, r)] {
                    1 => 'X',
                    2 => 'O',
                    _ => '.',
                });
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str(
            "  coloane: numărul de după literă, 1-9 (rândul e are 1-9, a are 1-5, i are 5-9)\n",
        );
        s.push_str(&format!(
            "  X: {} bile (a scos {})  O: {} bile (a scos {})   (câștigi scoțând {} bile)\n",
            self.b.iter().filter(|&&c| c == 1).count(),
            self.caps[0],
            self.b.iter().filter(|&&c| c == 2).count(),
            self.caps[1],
            WIN_CAPS
        ));
        s.push_str("  mutare: e5>ne (o bilă) sau c3-c5>se (grup de la prima la ultima bilă); direcții ne e se sw w nw\n");
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        if m.n == 1 {
            format!("{}>{}", name(m.first as usize), DN[m.d as usize])
        } else {
            let g = self.group(m);
            format!(
                "{}-{}>{}",
                name(g[0]),
                name(*g.last().unwrap()),
                DN[m.d as usize]
            )
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 600,
            max_depth: 6,
        }
    }

    fn hash_key(&self) -> Option<u64> {
        let mut h = polytabletop_core::hash_mix(
            3 << 40
                | (self.side.idx() as u64) << 32
                | (self.caps[0] as u64) << 24
                | (self.caps[1] as u64) << 16
                | self.quiet as u64,
        );
        for (i, &v) in self.b.iter().enumerate() {
            if v != 0 {
                h ^= polytabletop_core::hash_mix((v as u64) << 16 | i as u64);
            }
        }
        Some(h)
    }

    fn move_score(&self, m: &Mv) -> i32 {
        100 * self.captures_of(m) as i32 + 3 * m.n as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(s: &str) -> usize {
        let b = s.as_bytes();
        let r = 4 - (b[0] - b'a') as i8;
        let q = (b[1] - b'1') as i8 - 4;
        assert!(valid(q, r), "{s}");
        idx(q, r)
    }
    fn empty() -> Bile {
        Bile {
            b: [0; NC],
            caps: [0, 0],
            side: Player::One,
            quiet: 0,
        }
    }
    fn texts(g: &Bile) -> Vec<String> {
        g.legal_moves().iter().map(|m| g.format_move(m)).collect()
    }

    #[test]
    fn start_position_counts() {
        let g = Bile::new();
        assert_eq!(g.b.iter().filter(|&&c| c == 1).count(), 14);
        assert_eq!(g.b.iter().filter(|&&c| c == 2).count(), 14);
        assert!(!g.legal_moves().is_empty());
    }

    #[test]
    fn sumito_push_off_edge() {
        let mut g = empty();
        // pe rândul e (r = 0): 2 bile X împing o bilă O de la marginea din dreapta.
        g.b[at("e7")] = 1;
        g.b[at("e8")] = 1;
        g.b[at("e9")] = 2;
        g.b[at("a1")] = 2;
        let t = texts(&g);
        assert!(t.contains(&"e7-e8>e".to_string()), "{t:?}");
        let mv = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == "e7-e8>e")
            .unwrap();
        g.apply(&mv);
        assert_eq!(g.caps[0], 1);
        assert_eq!(g.b[at("e9")], 1);
    }

    #[test]
    fn equal_groups_cannot_push() {
        let mut g = empty();
        g.b[at("e4")] = 1;
        g.b[at("e5")] = 2;
        g.b[at("a1")] = 2;
        let t = texts(&g);
        assert!(!t.contains(&"e4>e".to_string()));
        assert!(t.contains(&"e4>w".to_string()));
    }

    #[test]
    fn side_step_needs_free_cells() {
        let mut g = empty();
        g.b[at("e4")] = 1;
        g.b[at("e5")] = 1;
        g.b[at("f4")] = 2; // blochează o destinație laterală
        let t = texts(&g);
        assert!(!t.contains(&"e4-e5>nw".to_string()));
        assert!(t.contains(&"e4-e5>ne".to_string()));
    }
}
