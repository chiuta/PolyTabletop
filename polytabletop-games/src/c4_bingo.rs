//! Familia „completează cartonașul” (variante originale C4, 2 jucători): mutarea forțată `scot` scoate două
//! piese din sac (zar re-însămânțat la `clone()`); mutezi `ia 1|2` o piesă pentru tine, cealaltă merge la adversar.
//! O piesă se acceptă doar dacă ai nevoie de ea (și, la unele jocuri, dacă ai deja piesa-premisă).
//! Cartonașele sunt deschise (informație publică). Câștigă cine își completează primul cartonașul.
use crate::c4_util::*;
use polytabletop_core::rng::Rng;

pub struct Spec {
    pub id: &'static str,
    pub title: &'static str,
    /// Tipuri de piese în sac.
    pub kinds: usize,
    /// Cartonaș fix: câte piese de fiecare tip trebuie (dacă `random` = 0).
    pub need: &'static [u8],
    /// Dacă > 0: fiecare jucător primește un cartonaș cu atâtea tipuri distincte, câte una.
    pub random: usize,
    /// Piesa de tip s>0 cere să ai deja o piesă de tip 0.
    pub chain: bool,
    pub names: &'static [&'static str],
}

#[derive(Clone)]
pub struct Bingo {
    spec: &'static Spec,
    dice: Dice,
    need: [Vec<u8>; 2],
    have: [Vec<u8>; 2],
    offer: Option<[u8; 2]>,
    turn: usize,
    plies: u32,
    last: String,
}

impl Bingo {
    pub fn new(spec: &'static Spec) -> Self {
        Self::with_seed(spec, seed_now())
    }
    pub fn with_seed(spec: &'static Spec, seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut need = [spec.need.to_vec(), spec.need.to_vec()];
        if spec.random > 0 {
            for n in need.iter_mut() {
                let mut ks: Vec<usize> = (0..spec.kinds).collect();
                shuf(&mut ks, &mut rng);
                *n = vec![0; spec.kinds];
                for &k in ks.iter().take(spec.random) {
                    n[k] = 1;
                }
            }
        }
        Bingo {
            spec,
            dice: Dice::new(seed ^ 0x5bd1),
            have: [vec![0; spec.kinds], vec![0; spec.kinds]],
            need,
            offer: None,
            turn: 0,
            plies: 0,
            last: String::new(),
        }
    }
    fn nm(&self, s: u8) -> &'static str {
        self.spec.names[s as usize % self.spec.names.len()]
    }
    fn accepts(&self, w: usize, s: u8) -> bool {
        let s = s as usize;
        self.have[w][s] < self.need[w][s] && (!self.spec.chain || s == 0 || self.have[w][0] > 0)
    }
    fn missing(&self, w: usize) -> i32 {
        (0..self.spec.kinds)
            .map(|k| (self.need[w][k] - self.have[w][k]) as i32)
            .sum()
    }
    fn done(&self, w: usize) -> bool {
        self.missing(w) == 0
    }
}

impl Mini for Bingo {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        pl(self.turn)
    }
    fn moves(&self) -> Vec<String> {
        match self.offer {
            None => vec!["scot".into()],
            Some(o) => {
                if o[0] == o[1] {
                    vec!["ia 1".into()]
                } else {
                    vec!["ia 1".into(), "ia 2".into()]
                }
            }
        }
    }
    fn step(&mut self, m: &str) {
        if m == "scot" {
            let a = self.dice.below(self.spec.kinds) as u8;
            let b = self.dice.below(self.spec.kinds) as u8;
            self.offer = Some([a, b]);
            return;
        }
        let o = self.offer.take().unwrap();
        let j = if m == "ia 1" { 0 } else { 1 };
        let (mine, other) = (o[j], o[1 - j]);
        let me = self.turn;
        if self.accepts(me, mine) {
            self.have[me][mine as usize] += 1;
        }
        if self.accepts(1 - me, other) {
            self.have[1 - me][other as usize] += 1;
        }
        self.last = format!(
            "J{} a luat {}, J{} a primit {}",
            me + 1,
            self.nm(mine),
            2 - me,
            self.nm(other)
        );
        self.turn = 1 - me;
        self.plies += 1;
    }
    fn result(&self) -> Option<Outcome> {
        if self.offer.is_none() || self.plies > 0 {
            match (self.done(0), self.done(1)) {
                (true, true) => return Some(Outcome::Draw),
                (true, false) => return Some(Outcome::Win(Player::One)),
                (false, true) => return Some(Outcome::Win(Player::Two)),
                _ => {}
            }
        }
        if self.plies > 600 {
            return Some(by_score(-self.missing(0), -self.missing(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        100 * (self.missing(1 - a) - self.missing(a))
    }
    fn show(&self) -> String {
        let mut s = format!("{}\n", self.spec.title);
        for w in 0..2 {
            let parts: Vec<String> = (0..self.spec.kinds)
                .filter(|&k| self.need[w][k] > 0)
                .map(|k| {
                    format!(
                        "{} {}/{}",
                        self.nm(k as u8),
                        self.have[w][k],
                        self.need[w][k]
                    )
                })
                .collect();
            s.push_str(&format!("J{}: {}\n", w + 1, parts.join(", ")));
        }
        if let Some(o) = self.offer {
            s.push_str(&format!(
                "Piese scoase: 1={}, 2={}\n",
                self.nm(o[0]),
                self.nm(o[1])
            ));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        s.push_str(&format!(
            "La mutare: {} (scot | ia 1 | ia 2)\n",
            pname(self.side())
        ));
        s
    }
}

pub static SPECS: &[Spec] = &[
    Spec {
        id: "plansa_cu_poze",
        title: "Cartonașul cu Poze",
        kinds: 10,
        need: &[],
        random: 6,
        chain: false,
        names: &[
            "soare",
            "lună",
            "floare",
            "pisică",
            "căsuță",
            "pește",
            "mașină",
            "copac",
            "balon",
            "stea",
        ],
    },
    Spec {
        id: "gandacelul_adunat",
        title: "Gândăcelul din Piese",
        kinds: 6,
        need: &[1, 1, 2, 2, 1, 2],
        random: 0,
        chain: true,
        names: &["corp", "cap", "ochi", "antenă", "gură", "picior"],
    },
];

pub fn make(id: &str) -> Box<dyn polytabletop_core::GameObj> {
    Box::new(B(Bingo::new(
        SPECS.iter().find(|s| s.id == id).expect("spec"),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plays() {
        for s in SPECS {
            check_c(&|sd| Bingo::with_seed(s, sd), 800);
        }
    }
}
