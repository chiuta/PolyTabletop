//! Sprouts (Marea Britanie, J. H. Conway și M. Paterson, 1967), 2 jucători, joc topologic.
//!
//! * Se pornește cu 4 puncte. Pe rând, un jucător unește două puncte (sau un punct cu el însuși,
//!   printr-o buclă) cu o curbă care nu atinge și nu taie nicio curbă existentă, apoi pune un
//!   punct nou pe curbă. Din fiecare punct pot pleca cel mult 3 curbe (2 „vieți” consumate de curba
//!   nouă: capetele ei); punctul nou are deja 2 curbe.
//! * Cine nu mai poate muta pierde (câștigă ultimul care a mutat). Jocul durează cel mult 11 mutări.
//!
//! Implementare combinatorie (fără desen): poziția e o listă de **regiuni**; fiecare regiune are
//! una sau mai multe **frontiere**, iar o frontieră e șirul ciclic al „colțurilor” punctelor
//! (un punct de grad d apare în d colțuri; un punct izolat apare singur). O curbă între două
//! colțuri din aceeași regiune e o mutare; dacă desparte regiunea în două, celelalte frontiere
//! pot ajunge în oricare dintre părți, fiecare alegere fiind o mutare distinctă.
//!
//! Mutări: `a-b` (punctele a și b; `a-a` = buclă), cu sufix `.1`, `.2`… când aceeași pereche are
//! mai multe variante topologice (poziția rezultată se vede în listă prin regiunile afișate).

use polytabletop_core::{AiStyle, Game, Outcome, Player};

type Boundary = Vec<u8>;
type Region = Vec<Boundary>;

const START: u8 = 4;

#[derive(Clone, PartialEq, Debug)]
pub struct Mv {
    text: String,
    regions: Vec<Region>,
    deg: Vec<u8>,
}

#[derive(Clone)]
pub struct Sprouts {
    regions: Vec<Region>,
    /// grad per punct (index = id, 0 neutilizat)
    deg: Vec<u8>,
    side: Player,
}

impl Default for Sprouts {
    fn default() -> Self {
        Self::new()
    }
}

fn rot(b: &[u8], i: usize) -> Vec<u8> {
    let mut v = b[i..].to_vec();
    v.extend_from_slice(&b[..i]);
    v
}

impl Sprouts {
    pub fn new() -> Self {
        let mut deg = vec![0u8];
        let mut b = Vec::new();
        for i in 1..=START {
            deg.push(0);
            b.push(vec![i]);
        }
        Sprouts {
            regions: vec![b],
            deg,
            side: Player::One,
        }
    }

    fn lives(&self, v: u8) -> u8 {
        3 - self.deg[v as usize]
    }

    /// Elimină colțurile punctelor moarte și regiunile fără mutări.
    fn prune(regions: Vec<Region>, deg: &[u8]) -> Vec<Region> {
        let mut out = Vec::new();
        for r in regions {
            let mut nr: Region = Vec::new();
            for b in r {
                let nb: Vec<u8> = b.into_iter().filter(|&v| deg[v as usize] < 3).collect();
                if !nb.is_empty() {
                    nr.push(nb);
                }
            }
            if !nr.is_empty() {
                out.push(nr);
            }
        }
        out
    }

    /// Există cel puțin o mutare? (două colțuri de puncte diferite într-o regiune sau un colț cu 2 vieți)
    fn has_move(&self) -> bool {
        for r in &self.regions {
            let corners: Vec<u8> = r.iter().flatten().copied().collect();
            if corners.iter().any(|&v| self.lives(v) >= 2) {
                return true;
            }
            if corners.iter().any(|&v| v != corners[0]) {
                return true;
            }
        }
        false
    }

    fn gen(&self) -> Vec<Mv> {
        let mut raw: Vec<(u8, u8, Vec<Region>, Vec<u8>)> = Vec::new();
        let m = self.deg.len() as u8;
        for (ri, r) in self.regions.iter().enumerate() {
            // toate colțurile: (frontieră, index)
            let mut cs = Vec::new();
            for (bi, b) in r.iter().enumerate() {
                for ci in 0..b.len() {
                    cs.push((bi, ci));
                }
            }
            for x in 0..cs.len() {
                for y in x..cs.len() {
                    let (b1, c1) = cs[x];
                    let (b2, c2) = cs[y];
                    let (v1, v2) = (r[b1][c1], r[b2][c2]);
                    if x == y {
                        if self.lives(v1) < 2 {
                            continue;
                        }
                    } else {
                        if v1 == v2 || self.lives(v1) < 1 || self.lives(v2) < 1 {
                            continue;
                        }
                    }
                    let mut deg = self.deg.clone();
                    deg.push(2);
                    deg[v1 as usize] += 1;
                    deg[v2 as usize] += 1;
                    let others: Vec<usize> = (0..r.len()).filter(|&k| k != b1 && k != b2).collect();
                    let mut results: Vec<Vec<Region>> = Vec::new();
                    if x == y {
                        // buclă
                        let b = &r[b1];
                        let inner = vec![v1, m];
                        let outer: Vec<u8> = if self.deg[v1 as usize] == 0 {
                            vec![v1, m]
                        } else {
                            let mut o = Vec::new();
                            for (k, &w) in b.iter().enumerate() {
                                if k == c1 {
                                    o.extend_from_slice(&[v1, m, v1]);
                                } else {
                                    o.push(w);
                                }
                            }
                            o
                        };
                        let oth: Vec<usize> = (0..r.len()).filter(|&k| k != b1).collect();
                        for mask in 0..(1usize << oth.len()) {
                            let mut r_in: Region = vec![inner.clone()];
                            let mut r_out: Region = vec![outer.clone()];
                            for (t, &k) in oth.iter().enumerate() {
                                if mask >> t & 1 == 1 {
                                    r_in.push(r[k].clone());
                                } else {
                                    r_out.push(r[k].clone());
                                }
                            }
                            let mut rs = self.regions.clone();
                            rs.remove(ri);
                            rs.push(r_in);
                            rs.push(r_out);
                            results.push(rs);
                        }
                    } else if b1 == b2 {
                        let b = &r[b1];
                        let mut p1: Vec<u8> = b[c1..=c2].to_vec();
                        p1.push(m);
                        let mut p2: Vec<u8> = b[c2..].to_vec();
                        p2.extend_from_slice(&b[..=c1]);
                        p2.push(m);
                        for mask in 0..(1usize << others.len()) {
                            let mut r1: Region = vec![p1.clone()];
                            let mut r2: Region = vec![p2.clone()];
                            for (t, &k) in others.iter().enumerate() {
                                if mask >> t & 1 == 1 {
                                    r1.push(r[k].clone());
                                } else {
                                    r2.push(r[k].clone());
                                }
                            }
                            let mut rs = self.regions.clone();
                            rs.remove(ri);
                            rs.push(r1);
                            rs.push(r2);
                            results.push(rs);
                        }
                    } else {
                        let walk = |b: &Vec<u8>, i: usize| -> Vec<u8> {
                            if b.len() == 1 && self.deg[b[0] as usize] == 0 {
                                vec![b[0]]
                            } else {
                                let mut w = rot(b, i);
                                w.push(b[i]);
                                w
                            }
                        };
                        let mut merged = walk(&r[b1], c1);
                        merged.push(m);
                        merged.extend(walk(&r[b2], c2));
                        merged.push(m);
                        let mut nr: Region = vec![merged];
                        for &k in &others {
                            nr.push(r[k].clone());
                        }
                        let mut rs = self.regions.clone();
                        rs.remove(ri);
                        rs.push(nr);
                        results.push(rs);
                    }
                    for rs in results {
                        raw.push((v1.min(v2), v1.max(v2), rs, deg.clone()));
                    }
                }
            }
        }
        // denumire + eliminare dubluri exacte
        let mut out: Vec<Mv> = Vec::new();
        let mut seen: Vec<(u8, u8, Vec<Region>)> = Vec::new();
        for (a, b, rs, deg) in raw {
            let rs = Self::prune(rs, &deg);
            let mut canon = rs.clone();
            for r in &mut canon {
                r.sort();
            }
            canon.sort();
            if seen
                .iter()
                .any(|(x, y, c)| *x == a && *y == b && *c == canon)
            {
                continue;
            }
            seen.push((a, b, canon));
            out.push(Mv {
                text: format!("{a}-{b}"),
                regions: rs,
                deg,
            });
        }
        // sufixe pentru texte duplicate
        let texts: Vec<String> = out.iter().map(|m| m.text.clone()).collect();
        let mut count: std::collections::HashMap<String, usize> = Default::default();
        for m in &mut out {
            let total = texts.iter().filter(|t| **t == m.text).count();
            if total > 1 {
                let c = count.entry(m.text.clone()).or_insert(0);
                *c += 1;
                m.text = format!("{}.{}", m.text, c);
            }
        }
        out
    }
}

impl Game for Sprouts {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if !self.has_move() {
            return Vec::new();
        }
        self.gen()
    }

    fn apply(&mut self, m: &Mv) {
        self.regions = m.regions.clone();
        self.deg = m.deg.clone();
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.has_move() {
            None
        } else {
            Some(Outcome::Win(self.side.other()))
        }
    }

    fn evaluate(&self, _p: Player) -> i32 {
        0
    }

    fn render(&self) -> String {
        let mut s = format!("Sprouts ({START} puncte la start) — regiuni și frontiere (punctele moarte sunt omise)\n");
        for (i, r) in self.regions.iter().enumerate() {
            let bs: Vec<String> = r
                .iter()
                .map(|b| {
                    format!(
                        "({})",
                        b.iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<_>>()
                            .join(" ")
                    )
                })
                .collect();
            s.push_str(&format!("  regiunea {}: {}\n", i + 1, bs.join(" ")));
        }
        let lv: Vec<String> = (1..self.deg.len() as u8)
            .filter(|&v| self.lives(v) > 0)
            .map(|v| format!("{v}:{}", self.lives(v)))
            .collect();
        s.push_str(&format!("Vieți rămase (punct:vieți): {}\n", lv.join(" ")));
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!(
                "Nu mai sunt mutări. Câștigă J{} (ultima mutare).\n",
                p.idx() + 1
            )),
            _ => s.push_str(&format!("La mutare: J{}\n", self.side.idx() + 1)),
        }
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        m.text.clone()
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 500,
            max_depth: 14,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn start_moves_and_vertex_budget() {
        let g = Sprouts::new();
        let m = g.legal_moves();
        // 6 perechi + 4 bucle × 4 repartiții ale celorlalte 3 puncte (în/afara buclei, până la simetria sferei)
        assert_eq!(m.len(), 22);
        let mut h = g.clone();
        h.apply(&m[0]);
        // 1-2 : punctele 1 și 2 au grad 1, punctul nou (5) grad 2
        assert_eq!(h.deg[5], 2);
        assert_eq!(h.deg.iter().map(|&d| d as u32).sum::<u32>(), 4);
    }

    #[test]
    fn games_last_at_most_3n_minus_1_moves() {
        for seed in 1..=25u64 {
            let mut g: Box<dyn polytabletop_core::GameObj> = Box::new(Sprouts::new());
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty(), "{}", g.render());
                g.play(&m[rng.below(m.len())]).unwrap();
                n += 1;
                assert!(n <= 11, "{}", g.render());
            }
            assert!(n >= 1);
        }
    }

    #[test]
    fn ai_plays_and_does_not_lose_to_random_always() {
        polytabletop_core::ai::set_time_budget_override_ms(Some(30));
        let mut g: Box<dyn polytabletop_core::GameObj> = Box::new(Sprouts::new());
        let mut rng = Rng::new(9);
        while g.outcome().is_none() {
            let m = if g.to_move() == Player::One {
                g.ai_move(rng.next_u64()).unwrap()
            } else {
                let l = g.legal_moves();
                l[rng.below(l.len())].clone()
            };
            g.play(&m).unwrap();
        }
    }
}
