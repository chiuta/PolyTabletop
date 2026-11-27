//! Familia „memorie” (variante originale C4, 2 jucători): cărți cu fața în jos, perechi de simboluri;
//! alegi `intorc i`, iar simbolul se dezvăluie la mutarea forțată `arata`. Ce s-a văzut e public
//! (`known`), AI-ul folosește doar cărțile deja văzute și nu citește fețele ascunse.
//! Moduri: `Pairs` (puncte pe pereche, fantomele dau punct adversarului), `Race` (perechea înaintează
//! pionul cu valoarea ei; fantomele te întorc), `Clock` (cooperativ: fiecare greșeală urcă fantoma pe turn;
//! dacă ajunge sus, pierdeți amândoi; altfel câștigă cine a găsit mai multe perechi).
use crate::c4_util::*;
use polytabletop_core::rng::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Pairs,
    Race(i32),
    Clock(i32),
}

pub struct Spec {
    pub id: &'static str,
    pub title: &'static str,
    pub pairs: usize,
    pub ghosts: usize,
    pub mode: Mode,
    pub names: &'static [&'static str],
}

#[derive(Clone)]
pub struct Memo {
    spec: &'static Spec,
    sym: Vec<u8>,
    known: Vec<bool>,
    taken: Vec<bool>,
    sel: Option<usize>,
    pend: Option<usize>,
    turn: usize,
    pts: [i32; 2],
    pos: [i32; 2],
    ghost: i32,
    plies: u32,
    last: String,
}

const G: u8 = 255;

impl Memo {
    pub fn new(spec: &'static Spec) -> Self {
        Self::with_seed(spec, seed_now())
    }
    pub fn with_seed(spec: &'static Spec, seed: u64) -> Self {
        let mut sym: Vec<u8> = (0..spec.pairs).flat_map(|i| [i as u8, i as u8]).collect();
        sym.extend(std::iter::repeat_n(G, spec.ghosts));
        let mut rng = Rng::new(seed);
        shuf(&mut sym, &mut rng);
        let n = sym.len();
        Memo {
            spec,
            sym,
            known: vec![false; n],
            taken: vec![false; n],
            sel: None,
            pend: None,
            turn: 0,
            pts: [0; 2],
            pos: [0; 2],
            ghost: 0,
            plies: 0,
            last: String::new(),
        }
    }
    fn nm(&self, s: u8) -> String {
        if s == G {
            "fantomă".into()
        } else {
            self.spec.names[s as usize % self.spec.names.len()].to_string()
        }
    }
    fn left(&self) -> usize {
        (0..self.sym.len())
            .filter(|&i| !self.taken[i] && self.sym[i] != G)
            .count()
    }
    fn pass(&mut self) {
        self.sel = None;
        self.turn = 1 - self.turn;
        self.plies += 1;
    }
}

impl Mini for Memo {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        pl(self.turn)
    }
    fn moves(&self) -> Vec<String> {
        if self.pend.is_some() {
            return vec!["arata".into()];
        }
        (0..self.sym.len())
            .filter(|&i| !self.taken[i] && self.sel != Some(i))
            .map(|i| format!("intorc {}", i + 1))
            .collect()
    }
    fn step(&mut self, m: &str) {
        if m != "arata" {
            self.pend = Some(m[7..].parse::<usize>().unwrap() - 1);
            return;
        }
        let me = self.turn;
        let i = self.pend.take().unwrap();
        self.known[i] = true;
        let s = self.sym[i];
        if s == G {
            self.last = format!("{}: fantomă!", i + 1);
            match self.spec.mode {
                Mode::Pairs => self.pts[1 - me] += 1,
                Mode::Race(_) => self.pos[me] = (self.pos[me] - 2).max(0),
                Mode::Clock(_) => self.ghost += 1,
            }
            self.pass();
            return;
        }
        match self.sel {
            None => self.sel = Some(i),
            Some(j) => {
                self.last = format!(
                    "{}={} și {}={}",
                    j + 1,
                    self.nm(self.sym[j]),
                    i + 1,
                    self.nm(s)
                );
                if self.sym[j] == s {
                    self.taken[i] = true;
                    self.taken[j] = true;
                    self.sel = None;
                    match self.spec.mode {
                        Mode::Race(_) => self.pos[me] += s as i32 % 3 + 1,
                        _ => self.pts[me] += 1,
                    }
                } else {
                    if let Mode::Clock(_) = self.spec.mode {
                        self.ghost += 1;
                    }
                    self.pass();
                }
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.pend.is_some() {
            return None;
        }
        match self.spec.mode {
            Mode::Race(len) => {
                for w in 0..2 {
                    if self.pos[w] >= len {
                        return Some(Outcome::Win(pl(w)));
                    }
                }
                if self.left() == 0 {
                    return Some(pts_outcome(self.pos));
                }
            }
            Mode::Clock(len) => {
                if self.ghost >= len {
                    return Some(Outcome::Draw);
                }
                if self.left() == 0 {
                    return Some(pts_outcome(self.pts));
                }
            }
            Mode::Pairs => {
                if self.left() == 0 {
                    return Some(pts_outcome(self.pts));
                }
            }
        }
        if self.plies > 500 {
            return Some(pts_outcome(self.pts));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let sc = |w: usize| match self.spec.mode {
            Mode::Race(_) => self.pos[w],
            _ => self.pts[w],
        };
        let mut v = 100 * (sc(a) - sc(1 - a));
        if let Some(i) = self.pend {
            let sign = if self.turn == a { 1 } else { -1 };
            let mut b = 0;
            if self.known[i] {
                if self.sym[i] == G {
                    b = -60;
                } else if let Some(j) = self.sel {
                    b = if self.sym[j] == self.sym[i] { 100 } else { -10 };
                } else {
                    let partner = (0..self.sym.len()).any(|k| {
                        k != i && self.known[k] && !self.taken[k] && self.sym[k] == self.sym[i]
                    });
                    b = if partner { 80 } else { -10 };
                }
            } else {
                b += 5;
            }
            v += sign * b;
        }
        v
    }
    fn show(&self) -> String {
        let mut s = format!("{}\n", self.spec.title);
        let mut row = String::new();
        for i in 0..self.sym.len() {
            let c = if self.taken[i] {
                format!("[{}:{}]", i + 1, self.nm(self.sym[i]))
            } else if self.sel == Some(i) {
                format!("[{}:{}!]", i + 1, self.nm(self.sym[i]))
            } else {
                format!("[{}:?]", i + 1)
            };
            row.push_str(&c);
            row.push(' ');
        }
        s.push_str(&row);
        s.push('\n');
        match self.spec.mode {
            Mode::Race(len) => s.push_str(&format!(
                "Pioni: J1 {}/{len}, J2 {}/{len}\n",
                self.pos[0], self.pos[1]
            )),
            Mode::Clock(len) => s.push_str(&format!(
                "Fantoma pe turn: {}/{len}. Perechi: J1 {}, J2 {}\n",
                self.ghost, self.pts[0], self.pts[1]
            )),
            Mode::Pairs => s.push_str(&format!("Puncte: J1 {}, J2 {}\n", self.pts[0], self.pts[1])),
        }
        if !self.last.is_empty() {
            s.push_str(&format!("Ultima întoarcere: {}\n", self.last));
        }
        s.push_str(&format!(
            "La mutare: {} (intorc i | arata)\n",
            pname(self.side())
        ));
        s
    }
}

pub static SPECS: &[Spec] = &[
    Spec {
        id: "chei_si_usi",
        title: "Cheile și Ușile",
        pairs: 8,
        ghosts: 0,
        mode: Mode::Pairs,
        names: &[
            "cheie roșie",
            "cheie verde",
            "cheie albastră",
            "cheie aurie",
            "lacăt mic",
            "lacăt mare",
            "ușă",
            "ladă",
        ],
    },
    Spec {
        id: "trepte_soaptelor",
        title: "Treptele Șoaptelor",
        pairs: 7,
        ghosts: 2,
        mode: Mode::Race(14),
        names: &[
            "lampă", "liliac", "pânză", "dovleac", "ceaun", "pisică", "coasă",
        ],
    },
    Spec {
        id: "stafii_comoara",
        title: "Stafiile și Comoara",
        pairs: 7,
        ghosts: 3,
        mode: Mode::Pairs,
        names: &[
            "monedă", "inel", "coroană", "cupă", "hartă", "busolă", "perlă",
        ],
    },
    Spec {
        id: "fantoma_de_pe_turn",
        title: "Fantoma de pe Turn",
        pairs: 6,
        ghosts: 0,
        mode: Mode::Clock(7),
        names: &["pană", "oglindă", "carte", "poțiune", "clopot", "cheie"],
    },
    Spec {
        id: "insula_cu_furtuni",
        title: "Insula cu Furtuni",
        pairs: 8,
        ghosts: 2,
        mode: Mode::Race(15),
        names: &[
            "nucă de cocos",
            "scoică",
            "crab",
            "palmier",
            "barcă",
            "peștișor",
            "stea de mare",
            "coral",
        ],
    },
];

pub fn make(id: &str) -> Box<dyn polytabletop_core::GameObj> {
    Box::new(B(Memo::new(
        SPECS.iter().find(|s| s.id == id).expect("spec"),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plays() {
        for s in SPECS {
            check_c(&|sd| Memo::with_seed(s, sd), 800);
        }
    }

    #[test]
    fn ai_ignores_hidden_faces() {
        let g = Memo::with_seed(&SPECS[0], 3);
        let mut h = g.clone();
        // schimbă două cărți necunoscute între ele: AI-ul trebuie să aleagă la fel
        h.sym.swap(0, 1);
        h.sym.swap(2, 5);
        for seed in 1..6 {
            assert_eq!(ai_of(&g, seed), ai_of(&h, seed));
        }
    }
}
