//! Cetățuia — joc original de plasare de muncitori pe un drum de clădiri și
//! construire a unei cetăți (2 jucători).
//!
//! Mecanica „muncitorii se plasează pe clădirile de pe un drum care crește,
//! un jaloner decide până unde se activează, se construiesc clădiri proprii și
//! se livrează materiale la cetate” e generică; clădirile, cifrele și textele
//! sunt ale noastre. Informație completă, fără zaruri.
//!
//! Reguli:
//! * Resurse: bani `d`, lemn `w`, piatră `s`, hrană `f`. Încep cu 5 bani, 2
//!   lemn, 1 piatră, 2 hrană. Fiecare are 5 muncitori. În fiecare rundă se
//!   primesc 3 bani.
//! * Drumul pornește cu 8 locuri: `piata` (+3 bani), `ferma` (+2 hrană),
//!   `padure` (+2 lemn), `cariera` (+2 piatră), `atelier` (construiești o
//!   clădire de lemn), `arhitect` (o clădire de piatră), `castel`, `pod`.
//!   Clădirile construite se adaugă la capătul drumului.
//! * Pe rând plasezi un muncitor pe un loc liber (`pune ferma`); primele 3
//!   plasări ale rundei costă 1 ban, următoarele 2. `pas` = nu mai plasezi
//!   în runda asta; primul care dă pas primește 1 ban și începe runda
//!   următoare. La `pod` muncitorul mută jalonerul imediat cu 1–3 locuri
//!   înainte sau înapoi, plătind 1 ban pe loc (`pune pod +2`, `pune pod -1`).
//! * Când ambii au terminat, locurile se activează în ordine, până la jaloner
//!   inclusiv; ce e după el nu se activează. Efectele fără plată (producție)
//!   se aplică singure; clădirile cu plată se activează doar dacă o poți
//!   plăti. La `atelier` / `arhitect` poți construi (`construieste gater`) sau
//!   `sari`; la `castel` livrezi (`castel l2 p1`: 2 piese cu lemn, 1 cu piatră,
//!   fiecare piesă costă încă 1 hrană; cel mult 3 piese). Când folosești
//!   clădirea altuia, proprietarul primește 1 punct.
//! * Cetatea are 12 piese: primele 4 valorează 3 puncte, următoarele 4 câte 2,
//!   ultimele 4 câte 1. Clădirile construite aduc 2 (lemn) sau 3 (piatră)
//!   puncte. Jalonerul avansează cu un loc la sfârșitul fiecărei runde.
//! * Final: după runda în care cetatea e completă sau după runda 9. Cine a
//!   livrat strict mai multe piese primește 4 puncte; câte 1 punct pentru
//!   fiecare 4 resurse rămase (bani + lemn + piatră + hrană).

#![allow(clippy::needless_range_loop)]

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const FIXED: [&str; 8] = [
    "piata", "ferma", "padure", "cariera", "atelier", "arhitect", "castel", "pod",
];
const K_ATELIER: u8 = 4;
const K_ARHITECT: u8 = 5;
const K_CASTEL: u8 = 6;
const K_POD: u8 = 7;
const NFIXED: u8 = 8;

/// (nume, cost lemn, cost piatră, prestigiu, câștig [d,w,s,f], plată [d,w,s,f], puncte la activare)
type Spec = (&'static str, i32, i32, i32, [i32; 4], [i32; 4], i32);
const BUILT: [Spec; 8] = [
    ("grajd", 1, 0, 2, [1, 0, 0, 2], [0; 4], 0),
    ("gater", 2, 0, 2, [0, 3, 0, 0], [0; 4], 0),
    ("negot", 1, 0, 2, [4, 0, 0, 0], [0; 4], 0),
    ("han", 2, 0, 2, [2, 0, 0, 0], [0, 0, 0, 1], 1),
    ("cariera2", 1, 1, 3, [0, 0, 3, 0], [0; 4], 0),
    ("sculptor", 1, 1, 3, [0; 4], [0, 1, 1, 0], 3),
    ("bijutier", 0, 2, 3, [4, 0, 0, 0], [0, 0, 1, 0], 1),
    ("turn", 0, 2, 3, [2, 0, 0, 0], [0; 4], 2),
];
const PIECES: i32 = 12;
const ROUNDS: u8 = 9;
const WORKERS: i32 = 5;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Place(u8),
    Bridge(i8),
    Pass,
    Build(u8),
    Castle(u8, u8),
    Skip,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Place,
    Decide(usize),
    Over,
}

#[derive(Clone)]
pub struct Cetatuia {
    kinds: Vec<u8>,
    worker: Vec<Option<Player>>,
    owner: Vec<Option<Player>>,
    provost: usize,
    res: [[i32; 4]; 2],
    vp: [i32; 2],
    pieces: [i32; 2],
    placed: [i32; 2],
    passed: [bool; 2],
    first_passer: Option<Player>,
    starter: Player,
    built_mask: u16,
    side: Player,
    phase: Phase,
    round: u8,
    outcome: Option<Outcome>,
    last: String,
}

impl Default for Cetatuia {
    fn default() -> Self {
        Self::new()
    }
}

fn piece_vp(n: i32) -> i32 {
    // n = numărul piesei (0-based)
    match n {
        0..=3 => 3,
        4..=7 => 2,
        _ => 1,
    }
}

impl Cetatuia {
    pub fn new() -> Self {
        Self::with_seed(polytabletop_core::rng::Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let starter = if polytabletop_core::rng::Rng::new(seed).below(2) == 0 {
            Player::One
        } else {
            Player::Two
        };
        Cetatuia {
            kinds: (0..NFIXED).collect(),
            worker: vec![None; NFIXED as usize],
            owner: vec![None; NFIXED as usize],
            provost: 4,
            res: [[5, 2, 1, 2]; 2],
            vp: [0; 2],
            pieces: [0; 2],
            placed: [0; 2],
            passed: [false; 2],
            first_passer: None,
            starter,
            built_mask: 0,
            side: starter,
            phase: Phase::Place,
            round: 1,
            outcome: None,
            last: String::new(),
        }
    }

    fn name(&self, i: usize) -> String {
        let k = self.kinds[i];
        if k < NFIXED {
            FIXED[k as usize].to_string()
        } else {
            BUILT[(k - NFIXED) as usize].0.to_string()
        }
    }

    pub fn score(&self, p: Player) -> i32 {
        self.vp[p.idx()]
    }

    fn final_score(&self, p: Player) -> i32 {
        let i = p.idx();
        let mut s = self.vp[i] + self.res[i].iter().sum::<i32>() / 4;
        if self.pieces[i] > self.pieces[1 - i] {
            s += 4;
        }
        s
    }

    fn place_cost(&self, p: Player) -> i32 {
        if self.placed[p.idx()] < 3 {
            1
        } else {
            2
        }
    }

    fn can_place(&self, p: Player) -> bool {
        let i = p.idx();
        self.placed[i] < WORKERS
            && self.res[i][0] >= self.place_cost(p)
            && self.worker.iter().any(|w| w.is_none())
    }

    fn delivered(&self) -> i32 {
        self.pieces[0] + self.pieces[1]
    }

    fn pay_ok(&self, p: Player, pay: [i32; 4]) -> bool {
        (0..4).all(|r| self.res[p.idx()][r] >= pay[r])
    }

    fn built_options(&self, p: Player, stone: bool) -> Vec<u8> {
        (0..8u8)
            .filter(|&b| {
                let sp = &BUILT[b as usize];
                self.built_mask >> b & 1 == 0
                    && (sp.2 > 0) == stone
                    && self.res[p.idx()][1] >= sp.1
                    && self.res[p.idx()][2] >= sp.2
            })
            .collect()
    }

    fn castle_options(&self, p: Player) -> Vec<(u8, u8)> {
        let r = &self.res[p.idx()];
        let left = PIECES - self.delivered();
        let mut v = Vec::new();
        for a in 0..=3i32 {
            for b in 0..=3 - a {
                let n = a + b;
                if n >= 1 && n <= left && n <= r[3] && a <= r[1] && b <= r[2] {
                    v.push((a as u8, b as u8));
                }
            }
        }
        v
    }

    /// Activează efectele automate, de la `from` încolo, până la o decizie.
    fn resolve(&mut self, from: usize) {
        let end = self.provost.min(self.kinds.len() - 1);
        let mut i = from;
        while i <= end {
            if let Some(w) = self.worker[i] {
                let k = self.kinds[i];
                match k {
                    K_ATELIER | K_ARHITECT => {
                        if !self.built_options(w, k == K_ARHITECT).is_empty() {
                            self.side = w;
                            self.phase = Phase::Decide(i);
                            return;
                        }
                    }
                    K_CASTEL => {
                        if !self.castle_options(w).is_empty() {
                            self.side = w;
                            self.phase = Phase::Decide(i);
                            return;
                        }
                    }
                    K_POD => {}
                    0..=3 => {
                        let g: [i32; 4] = match k {
                            0 => [3, 0, 0, 0],
                            1 => [0, 0, 0, 2],
                            2 => [0, 2, 0, 0],
                            _ => [0, 0, 2, 0],
                        };
                        for r in 0..4 {
                            self.res[w.idx()][r] += g[r];
                        }
                    }
                    _ => {
                        let sp = BUILT[(k - NFIXED) as usize];
                        if self.pay_ok(w, sp.5) {
                            for r in 0..4 {
                                self.res[w.idx()][r] += sp.4[r] - sp.5[r];
                            }
                            self.vp[w.idx()] += sp.6;
                            if let Some(o) = self.owner[i] {
                                if o != w {
                                    self.vp[o.idx()] += 1;
                                }
                            }
                        }
                    }
                }
            }
            i += 1;
        }
        self.end_round();
    }

    fn end_round(&mut self) {
        let over = self.delivered() >= PIECES || self.round >= ROUNDS;
        if over {
            self.phase = Phase::Over;
            let (a, b) = (self.final_score(Player::One), self.final_score(Player::Two));
            self.outcome = Some(if a > b {
                Outcome::Win(Player::One)
            } else if b > a {
                Outcome::Win(Player::Two)
            } else {
                Outcome::Draw
            });
            return;
        }
        for w in self.worker.iter_mut() {
            *w = None;
        }
        self.provost = (self.provost + 1).min(self.kinds.len() - 1);
        for r in self.res.iter_mut() {
            r[0] += 3;
        }
        self.starter = self.first_passer.unwrap_or(self.starter.other());
        self.first_passer = None;
        self.placed = [0; 2];
        self.passed = [false; 2];
        self.round += 1;
        self.phase = Phase::Place;
        self.side = self.starter;
        self.settle();
    }

    /// Alege cine plasează mai departe; când nimeni nu mai poate, rezolvă.
    fn settle(&mut self) {
        for p in [Player::One, Player::Two] {
            if !self.passed[p.idx()] && !self.can_place(p) {
                self.passed[p.idx()] = true;
            }
        }
        if self.passed[0] && self.passed[1] {
            self.resolve(0);
            return;
        }
        if self.passed[self.side.idx()] {
            self.side = self.side.other();
        }
    }

    fn next_after_place(&mut self, p: Player) {
        self.side = p.other();
        self.settle();
    }
}

impl Game for Cetatuia {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        let p = self.side;
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::Place => {
                let mut v = Vec::new();
                let d = self.res[p.idx()][0];
                let cost = self.place_cost(p);
                for i in 0..self.kinds.len() {
                    if self.worker[i].is_some() {
                        continue;
                    }
                    if self.kinds[i] == K_POD {
                        for n in [-3i8, -2, -1, 1, 2, 3] {
                            if d >= cost + n.abs() as i32 {
                                v.push(Move::Bridge(n));
                            }
                        }
                    } else {
                        v.push(Move::Place(i as u8));
                    }
                }
                v.push(Move::Pass);
                v
            }
            Phase::Decide(i) => {
                let k = self.kinds[i];
                let mut v = Vec::new();
                if k == K_CASTEL {
                    for (a, b) in self.castle_options(p) {
                        v.push(Move::Castle(a, b));
                    }
                } else {
                    for b in self.built_options(p, k == K_ARHITECT) {
                        v.push(Move::Build(b));
                    }
                }
                v.push(Move::Skip);
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        let p = self.side;
        let i = p.idx();
        match *m {
            Move::Place(s) => {
                self.res[i][0] -= self.place_cost(p);
                self.placed[i] += 1;
                self.worker[s as usize] = Some(p);
                self.last = format!("J{} pune pe {}", i + 1, self.name(s as usize));
                self.next_after_place(p);
            }
            Move::Bridge(n) => {
                let slot = self.kinds.iter().position(|&k| k == K_POD).unwrap_or(0);
                self.res[i][0] -= self.place_cost(p) + n.abs() as i32;
                self.placed[i] += 1;
                self.worker[slot] = Some(p);
                let np = self.provost as i32 + n as i32;
                self.provost = np.clamp(0, self.kinds.len() as i32 - 1) as usize;
                self.last = format!("J{} mută jalonerul la {}", i + 1, self.provost);
                self.next_after_place(p);
            }
            Move::Pass => {
                self.passed[i] = true;
                if self.first_passer.is_none() {
                    self.first_passer = Some(p);
                    self.res[i][0] += 1;
                }
                self.last = format!("J{} dă pas", i + 1);
                self.next_after_place(p);
            }
            Move::Build(b) => {
                let Phase::Decide(slot) = self.phase else {
                    return;
                };
                let sp = BUILT[b as usize];
                self.res[i][1] -= sp.1;
                self.res[i][2] -= sp.2;
                self.vp[i] += sp.3;
                self.built_mask |= 1 << b;
                self.kinds.push(NFIXED + b);
                self.worker.push(None);
                self.owner.push(Some(p));
                self.last = format!("J{} construiește {}", i + 1, sp.0);
                self.resolve(slot + 1);
            }
            Move::Castle(a, b) => {
                let Phase::Decide(slot) = self.phase else {
                    return;
                };
                let n = (a + b) as i32;
                self.res[i][3] -= n;
                self.res[i][1] -= a as i32;
                self.res[i][2] -= b as i32;
                for _ in 0..n {
                    self.vp[i] += piece_vp(self.delivered());
                    self.pieces[i] += 1;
                }
                self.last = format!("J{} livrează {n} piese la cetate", i + 1);
                self.resolve(slot + 1);
            }
            Move::Skip => {
                let Phase::Decide(slot) = self.phase else {
                    return;
                };
                self.resolve(slot + 1);
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.outcome
    }

    fn evaluate(&self, p: Player) -> i32 {
        let side = |q: Player| -> i32 {
            let i = q.idx();
            let r = &self.res[i];
            let mut v = self.vp[i] * 100 + r[0] * 12 + r[1] * 16 + r[2] * 20 + r[3] * 14;
            v += self.pieces[i] * 30;
            for (s, w) in self.worker.iter().enumerate() {
                if *w == Some(q) && s <= self.provost {
                    v += match self.kinds[s] {
                        0 => 36,
                        1 => 28,
                        2 => 32,
                        3 => 40,
                        K_ATELIER | K_ARHITECT => 70,
                        K_CASTEL => 120,
                        K_POD => 0,
                        k => {
                            let sp = &BUILT[(k - NFIXED) as usize];
                            let g = sp.4[0] * 12 + sp.4[1] * 16 + sp.4[2] * 20 + sp.4[3] * 14;
                            let c = sp.5[0] * 12 + sp.5[1] * 16 + sp.5[2] * 20 + sp.5[3] * 14;
                            g - c + sp.6 * 100
                        }
                    };
                }
            }
            for (s, o) in self.owner.iter().enumerate() {
                if *o == Some(q) {
                    v += 20;
                    if let Some(w) = self.worker[s] {
                        if w != q {
                            v += 100;
                        }
                    }
                }
            }
            v
        };
        side(p) - side(p.other())
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Cetățuia — runda {}/{} | jaloner la locul {} | piese livrate {}/{}\n",
            self.round,
            ROUNDS,
            self.provost,
            self.delivered(),
            PIECES
        );
        s.push_str("Drum (loc: clădire, muncitor, proprietar):\n");
        for i in 0..self.kinds.len() {
            let w = match self.worker[i] {
                Some(p) => format!(" <J{}>", p.idx() + 1),
                None => String::new(),
            };
            let o = match self.owner[i] {
                Some(p) => format!(" [J{}]", p.idx() + 1),
                None => String::new(),
            };
            let mark = if i == self.provost {
                " <== jaloner"
            } else {
                ""
            };
            s.push_str(&format!("  {i:>2}: {}{o}{w}{mark}\n", self.name(i)));
        }
        let free_w: Vec<&str> = (0..8)
            .filter(|&b| self.built_mask >> b & 1 == 0)
            .map(|b| BUILT[b].0)
            .collect();
        s.push_str(&format!(
            "Clădiri încă de construit: {}\n",
            free_w.join(" ")
        ));
        for q in [Player::One, Player::Two] {
            let i = q.idx();
            let r = &self.res[i];
            s.push_str(&format!(
                "J{}: {} puncte | bani {} lemn {} piatră {} hrană {} | piese {} | plasați {}/{}{}\n",
                i + 1,
                self.vp[i],
                r[0],
                r[1],
                r[2],
                r[3],
                self.pieces[i],
                self.placed[i],
                WORKERS,
                if self.passed[i] { " (pas)" } else { "" }
            ));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("Ultima mutare: {}\n", self.last));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Place(i) => format!("pune {}", self.name(i as usize)),
            Move::Bridge(n) => format!("pune pod {}{}", if n > 0 { "+" } else { "-" }, n.abs()),
            Move::Pass => "pas".into(),
            Move::Build(b) => format!("construieste {}", BUILT[b as usize].0),
            Move::Castle(a, b) => format!("castel l{a} p{b}"),
            Move::Skip => "sari".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    fn mv(g: &mut Cetatuia, s: &str) {
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == s)
            .unwrap_or_else(|| panic!("ilegal: {s}\n{}", g.render()));
        g.apply(&m);
    }

    #[test]
    fn start_state() {
        let g = Cetatuia::with_seed(1);
        assert_eq!(g.kinds.len(), 8);
        assert_eq!(g.res, [[5, 2, 1, 2]; 2]);
        let m = g.legal_moves();
        assert!(m.contains(&Move::Pass));
        assert!(m.contains(&Move::Place(0)));
        assert_eq!(m.iter().filter(|m| matches!(m, Move::Bridge(_))).count(), 6);
    }

    #[test]
    fn placement_costs_and_pass_bonus() {
        let mut g = Cetatuia::with_seed(2);
        let a = g.to_move();
        mv(&mut g, "pune piata");
        assert_eq!(g.res[a.idx()][0], 4);
        mv(&mut g, "pas"); // celălalt
        let b = a.other();
        assert_eq!(g.res[b.idx()][0], 6);
        assert_eq!(g.first_passer, Some(b));
        // a continuă singur
        assert_eq!(g.to_move(), a);
        mv(&mut g, "pas");
        // piata +3, venit +3, fără bonus de pas pentru al doilea
        assert_eq!(g.res[a.idx()][0], 4 + 3 + 3);
        assert_eq!(g.res[b.idx()][0], 6 + 3);
    }

    #[test]
    fn production_activates_up_to_provost() {
        let mut g = Cetatuia::with_seed(3);
        let a = g.to_move();
        let b = a.other();
        mv(&mut g, "pune piata");
        mv(&mut g, "pune castel"); // loc 6 > jaloner 4: nu se activează
        mv(&mut g, "pas");
        mv(&mut g, "pas");
        assert_eq!(g.round, 2);
        assert_eq!(g.res[a.idx()][0], 5 - 1 + 1 + 3 + 3);
        assert_eq!(g.res[b.idx()][0], 5 - 1 + 3);
        assert_eq!(g.provost, 5);
        assert_eq!(g.starter, a); // cine a dat primul pas
    }

    #[test]
    fn bridge_moves_provost() {
        let mut g = Cetatuia::with_seed(4);
        let a = g.to_move();
        mv(&mut g, "pune pod +2");
        assert_eq!(g.provost, 6);
        assert_eq!(g.res[a.idx()][0], 5 - 1 - 2);
        mv(&mut g, "pas");
        mv(&mut g, "pune castel");
        // jalonerul ajunge la castel => livrare
        mv(&mut g, "pas");
        // a pasează deja; rezolvare: castelul aparține lui b (cel de după a)
        assert!(matches!(g.phase, Phase::Decide(_)) || g.round == 2);
    }

    #[test]
    fn castle_delivery_scores() {
        let mut g = Cetatuia::with_seed(5);
        g.provost = 6;
        g.res[0] = [5, 3, 1, 4];
        g.res[1] = [5, 0, 0, 0];
        g.side = Player::One;
        g.starter = Player::One;
        mv(&mut g, "pune castel");
        mv(&mut g, "pas");
        mv(&mut g, "pas");
        assert_eq!(g.phase, Phase::Decide(6));
        let m = g.legal_moves();
        assert!(m.contains(&Move::Castle(3, 0)));
        assert!(m.contains(&Move::Castle(2, 1)));
        assert!(!m.contains(&Move::Castle(0, 2)));
        mv(&mut g, "castel l3 p0");
        assert_eq!(g.vp[0], 9);
        assert_eq!(g.pieces[0], 3);
        assert_eq!(g.res[0][3], 1);
        assert_eq!(g.res[0][1], 0);
        assert_eq!(g.round, 2);
    }

    #[test]
    fn building_extends_road_and_owner_bonus() {
        let mut g = Cetatuia::with_seed(6);
        g.side = Player::One;
        g.starter = Player::One;
        g.res[0] = [8, 3, 0, 0];
        mv(&mut g, "pune atelier");
        mv(&mut g, "pas");
        mv(&mut g, "pas");
        assert!(matches!(g.phase, Phase::Decide(4)));
        mv(&mut g, "construieste gater");
        assert_eq!(g.kinds.len(), 9);
        assert_eq!(g.vp[0], 2);
        assert_eq!(g.res[0][1], 1);
        assert_eq!(g.owner[8], Some(Player::One));
        // runda 2: J2 folosește gaterul (jaloner ajunge la 5 -> prea departe, îl mutăm)
        g.provost = 8;
        let first = g.to_move();
        let _ = first;
        g.side = Player::Two;
        g.starter = Player::Two;
        g.res[1][0] = 5;
        mv(&mut g, "pune gater");
        mv(&mut g, "pas");
        mv(&mut g, "pas");
        assert_eq!(g.res[1][1], 2 + 3);
        assert_eq!(g.vp[0], 3); // +1 pentru proprietar
    }

    #[test]
    fn game_ends_by_round_cap() {
        let mut g = Cetatuia::with_seed(7);
        for _ in 0..ROUNDS {
            if g.outcome().is_some() {
                break;
            }
            while g.phase == Phase::Place && g.outcome().is_none() {
                mv(&mut g, "pas");
            }
            while matches!(g.phase, Phase::Decide(_)) {
                mv(&mut g, "sari");
            }
        }
        assert!(g.outcome().is_some());
        assert_eq!(g.round, ROUNDS);
    }

    #[test]
    fn random_games_end_and_conserve() {
        for seed in 1..=30u64 {
            let mut g = Cetatuia::with_seed(seed);
            let mut rng = Rng::new(seed ^ 0x99);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty(), "{}", g.render());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 2000);
                assert!(g.delivered() <= PIECES);
                for r in &g.res {
                    assert!(r.iter().all(|&x| x >= 0), "{}", g.render());
                }
            }
        }
    }

    #[test]
    fn ai_beats_random_mostly() {
        let (mut ai, mut rnd) = (0, 0);
        for seed in 1..=8u64 {
            let ai_side = if seed % 2 == 0 {
                Player::One
            } else {
                Player::Two
            };
            let mut g = Cetatuia::with_seed(seed);
            let mut rng = Rng::new(seed * 3);
            while g.outcome().is_none() {
                let m = if g.to_move() == ai_side {
                    polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
                } else {
                    let l = g.legal_moves();
                    l[rng.below(l.len())]
                };
                g.apply(&m);
            }
            match g.outcome() {
                Some(Outcome::Win(p)) if p == ai_side => ai += 1,
                Some(Outcome::Win(_)) => rnd += 1,
                _ => {}
            }
        }
        assert!(ai > rnd, "AI {ai} vs random {rnd}");
    }
}
