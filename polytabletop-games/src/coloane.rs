//! Coloane (dame în turnuri), tablă 7×7 pe pătratele închise: fiecare jucător are 11 piese.
//! Piesele nu dispar la captură, ci se adună în „coloane": o coloană e condusă de piesa
//! de sus. Soldatul merge un pas în diagonală înainte; ajuns pe ultimul rând devine
//! ofițer (merge și ia în toate cele patru diagonale). Captura e obligatorie: sari peste
//! o coloană adversă vecină pe un pătrat liber de dincolo și iei doar piesa ei de sus,
//! care ajunge sub coloana ta; săriturile se înlănțuie (o coloană nu poate fi sărită de
//! două ori în aceeași tură; promovarea încheie tura). Pierde cine nu mai poate muta
//! (nu mai conduce nicio coloană sau e blocat); remiză după 80 de semi-mutări fără captură.
//!
//! Verificare surse: reguli publice ale jocurilor de dame în coloane (captură de vârf,
//! prizonieri la baza coloanei). Implementare proprie, denumiri și text originale.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const S: usize = 7;
const N: usize = S * S;
const QUIET_LIMIT: u16 = 80;
const DIAG: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

/// Coloană: piesa i (0 = vârful) pe biții 2i..2i+2; bit0 = proprietar (0 = J1), bit1 = ofițer.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
struct Col {
    len: u8,
    bits: u64,
}

impl Col {
    fn top(&self) -> Option<(usize, bool)> {
        if self.len == 0 {
            None
        } else {
            Some(((self.bits & 1) as usize, self.bits & 2 != 0))
        }
    }
    fn pop_top(&mut self) -> u64 {
        let p = self.bits & 3;
        self.bits >>= 2;
        self.len -= 1;
        p
    }
    fn push_bottom(&mut self, p: u64) {
        self.bits |= p << (2 * self.len as u32);
        self.len += 1;
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Mv {
    /// Pătratele parcurse (primul = plecarea).
    pub path: Vec<u8>,
    pub cap: bool,
}

#[derive(Clone)]
pub struct Coloane {
    b: [Col; N],
    side: Player,
    quiet: u16,
}

impl Default for Coloane {
    fn default() -> Self {
        Self::new()
    }
}

fn name(i: usize) -> String {
    polytabletop_core::square_name(i % S, i / S)
}

fn step(i: usize, d: (i32, i32), k: i32) -> Option<usize> {
    let (r, c) = ((i / S) as i32 + d.0 * k, (i % S) as i32 + d.1 * k);
    if r < 0 || c < 0 || r >= S as i32 || c >= S as i32 {
        None
    } else {
        Some(r as usize * S + c as usize)
    }
}

impl Coloane {
    pub fn new() -> Self {
        let mut b = [Col::default(); N];
        for r in 0..S {
            for c in 0..S {
                if (r + c) % 2 != 0 {
                    continue;
                }
                let i = r * S + c;
                if r <= 2 {
                    b[i] = Col { len: 1, bits: 0 };
                } else if r >= 4 {
                    b[i] = Col { len: 1, bits: 1 };
                }
            }
        }
        Coloane {
            b,
            side: Player::One,
            quiet: 0,
        }
    }

    fn fwd(pl: usize) -> i32 {
        if pl == 0 {
            1
        } else {
            -1
        }
    }

    fn last_rank(pl: usize) -> usize {
        if pl == 0 {
            S - 1
        } else {
            0
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn jumps(
        &self,
        pl: usize,
        officer: bool,
        origin: usize,
        cur: usize,
        caps: u64,
        path: &mut Vec<u8>,
        out: &mut Vec<Mv>,
    ) {
        let mut found = false;
        for &d in &DIAG {
            if !officer && d.0 != Self::fwd(pl) {
                continue;
            }
            let (Some(mid), Some(land)) = (step(cur, d, 1), step(cur, d, 2)) else {
                continue;
            };
            if caps >> mid & 1 == 1 {
                continue;
            }
            match self.b[mid].top() {
                Some((o, _)) if o != pl => {}
                _ => continue,
            }
            if self.b[land].len != 0 && land != origin {
                continue;
            }
            found = true;
            path.push(land as u8);
            if !officer && land / S == Self::last_rank(pl) {
                out.push(Mv {
                    path: path.clone(),
                    cap: true,
                });
            } else {
                let before = out.len();
                self.jumps(pl, officer, origin, land, caps | 1 << mid, path, out);
                if out.len() == before {
                    out.push(Mv {
                        path: path.clone(),
                        cap: true,
                    });
                }
            }
            path.pop();
        }
        let _ = found;
    }

    fn gen(&self) -> Vec<Mv> {
        let pl = self.side.idx();
        let mut caps = Vec::new();
        let mut steps = Vec::new();
        for i in 0..N {
            let Some((o, off)) = self.b[i].top() else {
                continue;
            };
            if o != pl {
                continue;
            }
            let mut path = vec![i as u8];
            self.jumps(pl, off, i, i, 0, &mut path, &mut caps);
            for &d in &DIAG {
                if !off && d.0 != Self::fwd(pl) {
                    continue;
                }
                if let Some(t) = step(i, d, 1) {
                    if self.b[t].len == 0 {
                        steps.push(Mv {
                            path: vec![i as u8, t as u8],
                            cap: false,
                        });
                    }
                }
            }
        }
        if caps.is_empty() {
            steps
        } else {
            caps
        }
    }

    fn counts(&self, pl: usize) -> (i32, i32) {
        let (mut s, mut o) = (0, 0);
        for c in &self.b {
            if let Some((p, off)) = c.top() {
                if p == pl {
                    if off {
                        o += 1
                    } else {
                        s += 1
                    }
                }
            }
        }
        (s, o)
    }
}

impl Game for Coloane {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.quiet >= QUIET_LIMIT {
            return Vec::new();
        }
        self.gen()
    }

    fn apply(&mut self, m: &Mv) {
        let pl = self.side.idx();
        let from = m.path[0] as usize;
        let mut col = self.b[from];
        self.b[from] = Col::default();
        if m.cap {
            for w in m.path.windows(2) {
                let (a, z) = (w[0] as usize, w[1] as usize);
                let mid = (a / S + z / S) / 2 * S + (a % S + z % S) / 2;
                let p = self.b[mid].pop_top();
                col.push_bottom(p);
            }
        }
        let to = *m.path.last().unwrap() as usize;
        if col.bits & 2 == 0 && to / S == Self::last_rank(pl) {
            col.bits |= 2;
        }
        self.b[to] = col;
        self.quiet = if m.cap { 0 } else { self.quiet + 1 };
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.quiet >= QUIET_LIMIT {
            return Some(Outcome::Draw);
        }
        if self.gen().is_empty() {
            return Some(Outcome::Win(self.side.other()));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut v = 0;
        for (i, c) in self.b.iter().enumerate() {
            let Some((o, off)) = c.top() else { continue };
            let sign = if o == me { 1 } else { -1 };
            let base = if off { 220 } else { 100 };
            let adv = if off {
                0
            } else if o == 0 {
                (i / S) as i32 * 4
            } else {
                (S - 1 - i / S) as i32 * 4
            };
            v += sign * (base + adv);
            // Prizonieri: piesele mele sub o coloană adversă (și invers).
            let mut bits = c.bits >> 2;
            for _ in 1..c.len {
                let owner = (bits & 1) as usize;
                v += if owner == me { 14 } else { -14 };
                bits >>= 2;
            }
        }
        v
    }

    fn render(&self) -> String {
        let mut s = String::from("   a  b  c  d  e  f  g\n");
        for r in (0..S).rev() {
            s.push_str(&format!("{}  ", r + 1));
            for c in 0..S {
                let col = &self.b[r * S + c];
                let cell = match col.top() {
                    None => {
                        if (r + c) % 2 == 0 {
                            " . ".to_string()
                        } else {
                            "   ".to_string()
                        }
                    }
                    Some((o, off)) => {
                        let ch = match (o, off) {
                            (0, false) => 'x',
                            (0, true) => 'X',
                            (_, false) => 'o',
                            (_, true) => 'O',
                        };
                        if col.len > 1 {
                            format!("{}{} ", ch, col.len)
                        } else {
                            format!("{}  ", ch)
                        }
                    }
                };
                s.push_str(&cell);
            }
            s.push('\n');
        }
        let (s1, o1) = self.counts(0);
        let (s2, o2) = self.counts(1);
        s.push_str(&format!(
            "  x/X (J1): {s1} soldați, {o1} ofițeri | o/O (J2): {s2} soldați, {o2} ofițeri\n"
        ));
        for i in 0..N {
            let c = &self.b[i];
            if c.len > 1 {
                let mut t = String::new();
                let mut bits = c.bits;
                for _ in 0..c.len {
                    t.push(match bits & 3 {
                        0 => 'x',
                        1 => 'o',
                        2 => 'X',
                        _ => 'O',
                    });
                    bits >>= 2;
                }
                s.push_str(&format!("  coloana {}: {} (de sus în jos)\n", name(i), t));
            }
        }
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        let sep = if m.cap { "x" } else { "-" };
        m.path
            .iter()
            .map(|&i| name(i as usize))
            .collect::<Vec<_>>()
            .join(sep)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 500,
            max_depth: 8,
        }
    }

    fn hash_key(&self) -> Option<u64> {
        let mut h = polytabletop_core::hash_mix(
            7 << 40 | (self.side.idx() as u64) << 32 | self.quiet as u64,
        );
        for (i, c) in self.b.iter().enumerate() {
            if c.len != 0 {
                h ^= polytabletop_core::hash_mix(
                    polytabletop_core::hash_mix(c.bits | (c.len as u64) << 48) ^ i as u64,
                );
            }
        }
        Some(h)
    }

    fn move_score(&self, m: &Mv) -> i32 {
        if m.cap {
            10 * (m.path.len() as i32 - 1)
        } else {
            0
        }
    }

    fn quiescence(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty() -> Coloane {
        Coloane {
            b: [Col::default(); N],
            side: Player::One,
            quiet: 0,
        }
    }
    fn at(s: &str) -> usize {
        let b = s.as_bytes();
        (b[1] - b'1') as usize * S + (b[0] - b'a') as usize
    }
    fn one(o: u64) -> Col {
        Col { len: 1, bits: o }
    }

    #[test]
    fn start_position() {
        let g = Coloane::new();
        assert_eq!(g.counts(0).0, 11);
        assert_eq!(g.counts(1).0, 11);
        assert!(g.legal_moves().iter().all(|m| !m.cap));
    }

    #[test]
    fn capture_takes_top_piece_under_column() {
        let mut g = empty();
        g.b[at("c3")] = one(0);
        g.b[at("d4")] = one(1);
        g.b[at("a1")] = one(1);
        let mv = g.legal_moves();
        assert_eq!(mv.len(), 1);
        assert_eq!(g.format_move(&mv[0]), "c3xe5");
        g.apply(&mv[0]);
        let c = g.b[at("e5")];
        assert_eq!(c.len, 2);
        assert_eq!(c.top(), Some((0, false)));
        assert_eq!(g.b[at("d4")].len, 0);
    }

    #[test]
    fn promotion_and_capture_is_mandatory() {
        let mut g = empty();
        g.b[at("b6")] = one(0);
        g.b[at("f2")] = one(0);
        g.b[at("g3")] = one(1);
        g.b[at("a1")] = one(1);
        g.apply(&Mv {
            path: vec![at("b6") as u8, at("a7") as u8],
            cap: false,
        });
        assert_eq!(g.b[at("a7")].top(), Some((0, true)));
        // J2 trebuie să ia, dacă poate: g3 peste f2 -> e1.
        let mv = g.legal_moves();
        assert!(mv.iter().all(|m| m.cap));
    }
}
