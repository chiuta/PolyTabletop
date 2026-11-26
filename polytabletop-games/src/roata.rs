//! Roata Destinului — parcurs de viață cu roată (variantă originală a jocurilor „drumul vieții”).
//!
//! * Fiecare jucător pornește de la Start și alege între **Facultate** (3 ani plătiți, apoi
//!   salariu mare) și **Ucenicie** (plecare rapidă, salariu mic). Drumul se unește, are o a
//!   doua bifurcație (**Afacere** riscantă sau **Salariu stabil**) și se termină la Pensie.
//! * Roata (1–6) este rotită automat și afișată în `render()`; mutarea este `avansez` (cu
//!   alegerea ramurii: `avansez facultate` / `avansez ucenicie`, apoi `avansez afacere` /
//!   `avansez salariu`, când drumul se desparte pe parcurs).
//!   Pe pătratele **Casă** poți cumpăra casa (`... cumpar` / `... nu`), dacă ai banii.
//! * Efectele de salariu (**Plată**, **Angajare**) și taxele de școlarizare se aplică de câte ori
//!   treci peste ele; celelalte (cheltuieli, premii, copii, casă) doar când te oprești pe ele.
//! * Cine ajunge la Pensie iese; celălalt joacă până la capăt. Scor final = bani + 30 (casa)
//!   + 8 pentru fiecare copil. Câștigă scorul mai mare; egal = remiză.
//!
//! Totul este determinist în afară de roată (afișată înainte), deci AI-ul nu vede nimic ascuns.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Start,
    Cost(i32),
    Job(i32),
    Pay,
    Event(i32),
    House,
    Family,
    Retire,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub kind: Kind,
    pub name: &'static str,
    pub next: Vec<usize>,
    /// Etichete pentru ramuri (când `next.len() == 2`).
    pub labels: [&'static str; 2],
}

pub const HOUSE_COST: i32 = 40;
pub const HOUSE_VALUE: i32 = 30;
pub const KID_VALUE: i32 = 8;
pub const START_MONEY: i32 = 10;

pub fn board() -> Vec<Node> {
    let mut v: Vec<Node> = Vec::new();
    let mut add = |kind, name| {
        v.push(Node {
            kind,
            name,
            next: Vec::new(),
            labels: ["", ""],
        });
        v.len() - 1
    };
    let start = add(Kind::Start, "Start");
    let s1 = add(Kind::Cost(-6), "Anul 1 de facultate");
    let s2 = add(Kind::Cost(-6), "Anul 2 de facultate");
    let s3 = add(Kind::Cost(-6), "Licență");
    let jh = add(Kind::Job(14), "Angajare: inginer (salariu 14)");
    let u1 = add(Kind::Event(2), "Ucenic: primul ban");
    let jl = add(Kind::Job(7), "Angajare: meșter (salariu 7)");
    // segmentul comun
    let m: Vec<usize> = vec![
        add(Kind::Pay, "Plată"),
        add(Kind::Event(4), "Premiu de merit (+4)"),
        add(Kind::House, "Casă de vânzare (40)"),
        add(Kind::Pay, "Plată"),
        add(Kind::Family, "Familie: un copil (-5)"),
        add(Kind::Event(-6), "Reparații (-6)"),
        add(Kind::Pay, "Plată"),
    ];
    // a doua bifurcație
    let fork = add(Kind::Event(0), "Răscruce: afacere sau salariu");
    let a1 = add(Kind::Event(-10), "Afacere: investiție (-10)");
    let a2 = add(Kind::Event(40), "Afacere: profit (+40)");
    let a3 = add(Kind::Event(-4), "Afacere: taxe (-4)");
    let b1 = add(Kind::Pay, "Salariu stabil: plată");
    let b2 = add(Kind::Pay, "Salariu stabil: plată");
    let b3 = add(Kind::Event(5), "Salariu stabil: bonus (+5)");
    let tail: Vec<usize> = vec![
        add(Kind::Pay, "Plată"),
        add(Kind::Family, "Familie: un copil (-5)"),
        add(Kind::Event(-8), "Impozit pe avere (-8)"),
        add(Kind::Pay, "Plată"),
        add(Kind::Event(6), "Chirie încasată (+6)"),
        add(Kind::Pay, "Plată"),
        add(Kind::Family, "Familie: un copil (-5)"),
        add(Kind::Pay, "Plată"),
    ];
    let retire = add(Kind::Retire, "Pensie");
    // legături
    v[start].next = vec![s1, u1];
    v[start].labels = ["facultate", "ucenicie"];
    v[s1].next = vec![s2];
    v[s2].next = vec![s3];
    v[s3].next = vec![jh];
    v[jh].next = vec![m[0]];
    v[u1].next = vec![jl];
    v[jl].next = vec![m[0]];
    for w in m.windows(2) {
        v[w[0]].next = vec![w[1]];
    }
    v[m[6]].next = vec![fork];
    v[fork].next = vec![a1, b1];
    v[fork].labels = ["afacere", "salariu"];
    v[a1].next = vec![a2];
    v[a2].next = vec![a3];
    v[a3].next = vec![tail[0]];
    v[b1].next = vec![b2];
    v[b2].next = vec![b3];
    v[b3].next = vec![tail[0]];
    for w in tail.windows(2) {
        v[w[0]].next = vec![w[1]];
    }
    v[tail[7]].next = vec![retire];
    v
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Route {
    /// Alegerile făcute la bifurcații (0/1), în ordine.
    pub choices: [u8; 4],
    pub n: u8,
    pub buy: Option<bool>,
}

#[derive(Clone)]
pub struct Roata {
    board: Vec<Node>,
    pos: [usize; 2],
    money: [i32; 2],
    salary: [i32; 2],
    house: [bool; 2],
    kids: [i32; 2],
    retired: [bool; 2],
    side: Player,
    roll: u8,
    rng: Rng,
    log: String,
}

impl Default for Roata {
    fn default() -> Self {
        Self::new()
    }
}

impl Roata {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let roll = rng.d6();
        Roata {
            board: board(),
            pos: [0; 2],
            money: [START_MONEY; 2],
            salary: [0; 2],
            house: [false; 2],
            kids: [0; 2],
            retired: [false; 2],
            side: Player::One,
            roll,
            rng,
            log: String::new(),
        }
    }

    pub fn score(&self, p: Player) -> i32 {
        let i = p.idx();
        self.money[i] + if self.house[i] { HOUSE_VALUE } else { 0 } + KID_VALUE * self.kids[i]
    }

    pub fn money(&self, p: Player) -> i32 {
        self.money[p.idx()]
    }

    /// Toate traseele posibile pentru zarul curent (cu alegerile de la bifurcații).
    fn routes(&self) -> Vec<Route> {
        let mut out = Vec::new();
        let i = self.side.idx();
        self.walk(
            self.pos[i],
            self.roll,
            Route {
                choices: [0; 4],
                n: 0,
                buy: None,
            },
            self.money[i],
            &mut out,
        );
        out
    }

    fn walk(&self, at: usize, left: u8, r: Route, money: i32, out: &mut Vec<Route>) {
        // `at` este pătratul curent; mai avem `left` pași.
        if left == 0 || self.board[at].kind == Kind::Retire {
            if self.board[at].kind == Kind::House && left == 0 {
                // simulăm banii aproximativ: cheltuielile apar la aterizare
                let can = !self.house[self.side.idx()] && money >= HOUSE_COST;
                if can {
                    let mut a = r;
                    a.buy = Some(true);
                    out.push(a);
                    let mut b = r;
                    b.buy = Some(false);
                    out.push(b);
                    return;
                }
            }
            out.push(r);
            return;
        }
        let node = &self.board[at];
        if node.next.len() == 1 {
            let nx = node.next[0];
            let mo = money + self.pass_gain(nx, left == 1, self.salary_after(nx));
            self.walk(nx, left - 1, r, mo, out);
        } else {
            for (k, &nx) in node.next.iter().enumerate() {
                let mut r2 = r;
                r2.choices[r2.n as usize] = k as u8;
                r2.n += 1;
                let mo = money + self.pass_gain(nx, left == 1, self.salary_after(nx));
                self.walk(nx, left - 1, r2, mo, out);
            }
        }
    }

    fn salary_after(&self, nx: usize) -> i32 {
        match self.board[nx].kind {
            Kind::Job(s) => s,
            _ => self.salary[self.side.idx()],
        }
    }

    /// Câștigul de bani la intrarea pe `nx` (doar aproximare pentru verificarea cumpărării casei).
    fn pass_gain(&self, nx: usize, landing: bool, salary: i32) -> i32 {
        match self.board[nx].kind {
            Kind::Pay | Kind::Job(_) => salary,
            Kind::Cost(c) => c,
            Kind::Event(c) if landing => c,
            Kind::Family if landing => -5,
            _ => 0,
        }
    }

    fn route_text(&self, r: &Route) -> String {
        let mut s = String::from("avansez");
        // reparcurgem traseul pentru a eticheta alegerile
        let mut at = self.pos[self.side.idx()];
        let mut left = self.roll;
        let mut ci = 0;
        while left > 0 && self.board[at].kind != Kind::Retire {
            let node = &self.board[at];
            let nx = if node.next.len() == 1 {
                node.next[0]
            } else {
                let k = r.choices[ci] as usize;
                ci += 1;
                s.push(' ');
                s.push_str(node.labels[k]);
                node.next[k]
            };
            at = nx;
            left -= 1;
        }
        match r.buy {
            Some(true) => s.push_str(" cumpar"),
            Some(false) => s.push_str(" nu"),
            None => {}
        }
        s
    }
}

impl Game for Roata {
    type Move = Route;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Route> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        self.routes()
    }

    fn apply(&mut self, r: &Route) {
        let i = self.side.idx();
        let mut at = self.pos[i];
        let mut left = self.roll;
        let mut ci = 0;
        let mut log = format!("Jucătorul {}: roata {}", i + 1, self.roll);
        while left > 0 && self.board[at].kind != Kind::Retire {
            let node = &self.board[at];
            at = if node.next.len() == 1 {
                node.next[0]
            } else {
                let k = r.choices[ci] as usize;
                ci += 1;
                node.next[k]
            };
            left -= 1;
            let (kind, name) = (self.board[at].kind, self.board[at].name);
            match kind {
                Kind::Job(s) => {
                    self.salary[i] = s;
                    self.money[i] += s;
                    log.push_str(&format!(", {name} (+{s})"));
                }
                Kind::Pay => {
                    self.money[i] += self.salary[i];
                    log.push_str(&format!(", plată +{}", self.salary[i]));
                }
                Kind::Cost(c) => {
                    self.money[i] += c;
                    log.push_str(&format!(", {name} ({c})"));
                }
                _ => {}
            }
            if left == 0 || kind == Kind::Retire {
                match kind {
                    Kind::Event(c) => {
                        self.money[i] += c;
                        log.push_str(&format!(", {name}"));
                    }
                    Kind::Family => {
                        self.money[i] -= 5;
                        self.kids[i] += 1;
                        log.push_str(", s-a născut un copil (-5)");
                    }
                    Kind::House => {
                        if r.buy == Some(true) {
                            self.money[i] -= HOUSE_COST;
                            self.house[i] = true;
                            log.push_str(", ai cumpărat casa (-40)");
                        } else {
                            log.push_str(", treci pe lângă casă");
                        }
                    }
                    Kind::Retire => {
                        self.retired[i] = true;
                        log.push_str(", ieși la pensie");
                    }
                    Kind::Start | Kind::Cost(_) | Kind::Job(_) | Kind::Pay => {}
                }
            }
        }
        self.pos[i] = at;
        log.push_str(&format!(
            " → «{}»; bani {}.",
            self.board[at].name, self.money[i]
        ));
        self.log = log;
        if !self.retired[self.side.other().idx()] {
            self.side = self.side.other();
        }
        self.roll = self.rng.d6();
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.retired[0] && self.retired[1] {
            let (a, b) = (self.score(Player::One), self.score(Player::Two));
            Some(match a.cmp(&b) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            })
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        let f = |q: Player| {
            let i = q.idx();
            // un salariu mai mare valorează mai mult cât mai e de mers
            let left = (self.board.len() - self.pos[i]) as i32;
            self.score(q) + self.salary[i] * left / 7
        };
        f(p) - f(p.other())
    }

    fn render(&self) -> String {
        let mut s = String::from("Roata Destinului\n");
        for (k, name) in ["Jucător 1 (X)", "Jucător 2 (O)"].iter().enumerate() {
            let p = if k == 0 { Player::One } else { Player::Two };
            s.push_str(&format!(
                "{name}: «{}»  bani {}  salariu {}  casă {}  copii {}  scor {}{}\n",
                self.board[self.pos[k]].name,
                self.money[k],
                self.salary[k],
                if self.house[k] { "da" } else { "nu" },
                self.kids[k],
                self.score(p),
                if self.retired[k] { "  [pensionat]" } else { "" }
            ));
        }
        if self.outcome().is_none() {
            s.push_str(&format!(
                "La mutare: jucătorul {}   Roata: {}\nMutări: ",
                self.side.idx() + 1,
                self.roll
            ));
            let t: Vec<String> = self.routes().iter().map(|r| self.route_text(r)).collect();
            s.push_str(&t.join(" | "));
            s.push('\n');
        }
        s.push_str(self.log.trim());
        s.push('\n');
        s
    }

    fn format_move(&self, m: &Route) -> String {
        self.route_text(m)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_is_a_dag_ending_in_retire() {
        let b = board();
        for (i, n) in b.iter().enumerate() {
            if n.kind == Kind::Retire {
                assert!(n.next.is_empty());
            } else {
                assert!(!n.next.is_empty() && n.next.len() <= 2, "{}", n.name);
                for &x in &n.next {
                    assert!(x > i || n.kind == Kind::Start, "nod {i} -> {x}");
                }
            }
        }
        assert_eq!(b.iter().filter(|n| n.kind == Kind::Retire).count(), 1);
    }

    #[test]
    fn rules_text_uses_real_move_labels() {
        let rules = crate::playable()
            .into_iter()
            .find(|p| p.id == "roata")
            .unwrap()
            .rules;
        // Etichetele din text (după `avansez `) trebuie să fie cele reale.
        let in_rules: std::collections::BTreeSet<&str> = rules
            .split(|c: char| !(c.is_alphanumeric() || c == ' '))
            .filter_map(|s| s.trim().strip_prefix("avansez "))
            .collect();
        let mut seen = std::collections::BTreeSet::new();
        for seed in 1..=40u64 {
            let mut g = Roata::with_seed(seed);
            while g.outcome().is_none() {
                let m = polytabletop_core::GameObj::legal_moves(&g);
                for t in &m {
                    for w in t.split_whitespace().skip(1) {
                        seen.insert(w.to_string());
                    }
                }
                let pick = m
                    .iter()
                    .find(|t| t.contains("cumpar"))
                    .unwrap_or(&m[(seed as usize) % m.len()])
                    .clone();
                polytabletop_core::GameObj::play(&mut g, &pick).unwrap();
            }
        }
        // Pătratul Casă, cu bani: apare și `cumpar`.
        let mut h = Roata::with_seed(1);
        let house = h.board.iter().position(|n| n.kind == Kind::House).unwrap();
        h.pos[0] = house - 1;
        h.roll = 1;
        h.money[0] = 100;
        for t in polytabletop_core::GameObj::legal_moves(&h) {
            for w in t.split_whitespace().skip(1) {
                seen.insert(w.to_string());
            }
        }
        for w in ["facultate", "ucenicie", "afacere", "salariu", "cumpar"] {
            assert!(
                seen.contains(w),
                "eticheta {w} nu apare în mutări: {seen:?}"
            );
            assert!(in_rules.contains(w), "{w} lipsește din rules: {in_rules:?}");
        }
        assert!(!rules.contains("stanga") && !rules.contains("dreapta"));
        for w in &in_rules {
            assert!(seen.contains(*w), "token inexistent: {w}");
        }
    }

    #[test]
    fn fork_at_start_gives_two_labeled_moves() {
        let mut g = Roata::with_seed(1);
        g.roll = 1;
        let m = polytabletop_core::GameObj::legal_moves(&g);
        assert_eq!(
            m,
            vec![
                "avansez facultate".to_string(),
                "avansez ucenicie".to_string()
            ]
        );
    }

    #[test]
    fn moves_are_unique_and_playable() {
        for seed in 1..=20u64 {
            let mut g = Roata::with_seed(seed);
            while g.outcome().is_none() {
                let mut m = polytabletop_core::GameObj::legal_moves(&g);
                let n = m.len();
                m.sort();
                m.dedup();
                assert_eq!(n, m.len());
                let pick = m[(seed as usize + n) % m.len()].clone();
                polytabletop_core::GameObj::play(&mut g, &pick).unwrap();
            }
        }
    }

    #[test]
    fn study_path_costs_and_raises_salary() {
        let mut g = Roata::with_seed(3);
        g.roll = 4;
        polytabletop_core::GameObj::play(&mut g, "avansez facultate").unwrap();
        // facultate: 3 ani -18, angajare +14 la trecere
        assert_eq!(g.money[0], START_MONEY - 18 + 14);
        assert_eq!(g.salary[0], 14);
        assert_eq!(g.board[g.pos[0]].name, "Angajare: inginer (salariu 14)");
    }

    #[test]
    fn house_can_only_be_bought_with_money() {
        let mut g = Roata::with_seed(1);
        let house = g.board.iter().position(|n| n.kind == Kind::House).unwrap();
        g.pos[0] = house - 1;
        g.roll = 1;
        g.money[0] = 10;
        assert_eq!(polytabletop_core::GameObj::legal_moves(&g).len(), 1);
        g.money[0] = 100;
        let m = polytabletop_core::GameObj::legal_moves(&g);
        assert_eq!(m.len(), 2);
        polytabletop_core::GameObj::play(&mut g, "avansez cumpar").unwrap();
        assert!(g.house[0]);
        assert_eq!(g.money[0], 60);
    }

    #[test]
    fn retired_player_is_skipped_and_game_ends() {
        let mut g = Roata::with_seed(1);
        let last = g.board.len() - 1;
        g.pos[0] = last - 1;
        g.roll = 3;
        polytabletop_core::GameObj::play(&mut g, "avansez").unwrap();
        assert!(g.retired[0]);
        assert_eq!(g.side, Player::Two);
        let mut n = 0;
        while g.outcome().is_none() {
            assert_eq!(g.side, Player::Two);
            let m = polytabletop_core::GameObj::legal_moves(&g);
            polytabletop_core::GameObj::play(&mut g, &m[0]).unwrap();
            n += 1;
            assert!(n < 200);
        }
        assert!(g.outcome().is_some());
    }

    #[test]
    fn final_score_decides() {
        let mut g = Roata::with_seed(1);
        g.retired = [true, true];
        g.money = [10, 10];
        g.kids = [0, 1];
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
        g.kids = [1, 1];
        assert_eq!(g.outcome(), Some(Outcome::Draw));
    }
}
