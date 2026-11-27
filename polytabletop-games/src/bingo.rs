//! Bingo și Lotería: două jocuri cu „tragere” automată, pe un același motor.
//!
//! * Fiecare jucător are o fișă; numerele/imaginile se trag dintr-un sac
//!   amestecat cu `Rng`-ul jocului. Tragerea e o mutare separată (`trag`, făcută
//!   de Jucătorul 1 pentru ambii), ca AI-ul să nu vadă ce urmează.
//! * După fiecare tragere, cine are elementul pe fișă trebuie să-l `marchez`;
//!   după marcare, cine are o combinație câștigătoare poate `strig` sau
//!   `continui`. Ordinea: J1 marchează, J2 marchează, J1 strigă?, J2 strigă?.
//!   Cine strigă singur câștigă; dacă strigă amândoi (aceeași tragere) =
//!   remiză; dacă sacul se golește fără strigăt = remiză.
//! * Bingo: fișă 5 × 5 cu coloanele B 1–15, I 16–30, N 31–45 (centru liber),
//!   G 46–60, O 61–75; combinații: rând, coloană, diagonală.
//! * Lotería: tablă 4 × 4 cu 16 din cele 54 de imagini ale noastre (nume
//!   proprii, nu cele ale vreunui set comercial); combinații: rând, coloană,
//!   diagonală, cele patru colțuri sau pătratul din centru 2 × 2.
//!
//! Informație ascunsă: `render()` arată doar fișa Jucătorului 1 (fișa AI-ului
//! apare la final). AI-ul își citește doar fișa lui și strigă când are
//! combinație.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Bingo,
    Loteria,
}

pub const LOTERIA_PICTURES: [&str; 54] = [
    "Soarele",
    "Luna",
    "Steaua",
    "Norul",
    "Ploaia",
    "Fulgerul",
    "Curcubeul",
    "Vântul",
    "Muntele",
    "Râul",
    "Pădurea",
    "Floarea",
    "Ciuperca",
    "Frunza",
    "Albina",
    "Fluturele",
    "Cocorul",
    "Bufnița",
    "Vulpea",
    "Ursul",
    "Lupul",
    "Calul",
    "Peștele",
    "Broasca",
    "Melcul",
    "Pisica",
    "Câinele",
    "Barca",
    "Podul",
    "Casa",
    "Turnul",
    "Clopotul",
    "Cheia",
    "Lacătul",
    "Lampa",
    "Oglinda",
    "Cartea",
    "Pana",
    "Coroana",
    "Sabia",
    "Scutul",
    "Harta",
    "Busola",
    "Ceasul",
    "Ulciorul",
    "Pâinea",
    "Mărul",
    "Strugurele",
    "Lămâia",
    "Dovleacul",
    "Ancora",
    "Roata",
    "Toba",
    "Vioara",
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Draw,
    Mark,
    Shout,
    Continue,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Draw,
    Mark(Player),
    Claim(Player),
    Over,
}

#[derive(Clone)]
pub struct CallGame {
    kind: Kind,
    cards: [Vec<u8>; 2],
    marks: [Vec<bool>; 2],
    bag: Vec<u8>, // se trage de la sfârșit
    drawn: Vec<u8>,
    phase: Phase,
    claims: [bool; 2],
    result: Option<Outcome>,
    /// `Some(p)` = render shows `p`'s card; `None` = hot-seat (the mover's).
    viewer: Option<Player>,
}

impl CallGame {
    pub fn bingo() -> Self {
        Self::bingo_seed(Rng::from_clock().next_u64())
    }

    pub fn loteria() -> Self {
        Self::loteria_seed(Rng::from_clock().next_u64())
    }

    pub fn bingo_seed(seed: u64) -> Self {
        Self::make(Kind::Bingo, seed)
    }

    pub fn loteria_seed(seed: u64) -> Self {
        Self::make(Kind::Loteria, seed)
    }

    fn make(kind: Kind, seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let top: u8 = match kind {
            Kind::Bingo => 75,
            Kind::Loteria => 54,
        };
        let mut bag: Vec<u8> = (1..=top).collect();
        shuffle(&mut bag, &mut rng);
        let cards = [make_card(kind, &mut rng), make_card(kind, &mut rng)];
        let marks = [
            cards[0].iter().map(|&v| v == 0).collect(),
            cards[1].iter().map(|&v| v == 0).collect(),
        ];
        CallGame {
            kind,
            cards,
            marks,
            bag,
            drawn: Vec::new(),
            phase: Phase::Draw,
            claims: [false; 2],
            result: None,
            viewer: Some(Player::One),
        }
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }

    pub fn drawn_count(&self) -> usize {
        self.drawn.len()
    }

    pub fn card(&self, p: Player) -> &[u8] {
        &self.cards[p.idx()]
    }

    pub fn has_line(&self, p: Player) -> bool {
        let m = &self.marks[p.idx()];
        patterns(self.kind)
            .iter()
            .any(|pat| pat.iter().all(|&c| m[c]))
    }

    fn side_len(&self) -> usize {
        match self.kind {
            Kind::Bingo => 5,
            Kind::Loteria => 4,
        }
    }

    fn name(&self, v: u8) -> String {
        match self.kind {
            Kind::Bingo => {
                let l = ['B', 'I', 'N', 'G', 'O'][((v - 1) / 15) as usize];
                format!("{l}{v}")
            }
            Kind::Loteria => format!("{} ({})", LOTERIA_PICTURES[v as usize - 1], v),
        }
    }

    fn current(&self) -> Option<u8> {
        self.drawn.last().copied()
    }

    fn needs_mark(&self, p: Player) -> bool {
        match self.current() {
            Some(v) => self.cards[p.idx()]
                .iter()
                .zip(&self.marks[p.idx()])
                .any(|(&c, &m)| c == v && !m),
            None => false,
        }
    }

    /// Trece la următoarea fază în care chiar există o decizie.
    fn advance(&mut self, from: Phase) {
        let mut ph = from;
        loop {
            match ph {
                Phase::Mark(Player::One) => {
                    if self.needs_mark(Player::One) {
                        break;
                    }
                    ph = Phase::Mark(Player::Two);
                }
                Phase::Mark(Player::Two) => {
                    if self.needs_mark(Player::Two) {
                        break;
                    }
                    ph = Phase::Claim(Player::One);
                }
                Phase::Claim(Player::One) => {
                    if self.has_line(Player::One) {
                        break;
                    }
                    ph = Phase::Claim(Player::Two);
                }
                Phase::Claim(Player::Two) => {
                    if self.has_line(Player::Two) {
                        break;
                    }
                    self.finish_claims();
                    return;
                }
                _ => break,
            }
        }
        self.phase = ph;
    }

    fn to_move_of(ph: Phase) -> Player {
        match ph {
            Phase::Mark(p) | Phase::Claim(p) => p,
            _ => Player::One,
        }
    }
}

fn shuffle<T>(v: &mut [T], rng: &mut Rng) {
    for i in (1..v.len()).rev() {
        let j = rng.below(i + 1);
        v.swap(i, j);
    }
}

fn make_card(kind: Kind, rng: &mut Rng) -> Vec<u8> {
    match kind {
        Kind::Bingo => {
            let mut card = vec![0u8; 25];
            for col in 0..5usize {
                let mut pool: Vec<u8> = (1..=15).map(|k| col as u8 * 15 + k).collect();
                shuffle(&mut pool, rng);
                for row in 0..5usize {
                    card[row * 5 + col] = pool[row];
                }
            }
            card[12] = 0; // centru liber
            card
        }
        Kind::Loteria => {
            let mut pool: Vec<u8> = (1..=54).collect();
            shuffle(&mut pool, rng);
            pool.truncate(16);
            pool
        }
    }
}

fn patterns(kind: Kind) -> Vec<Vec<usize>> {
    let n = match kind {
        Kind::Bingo => 5,
        Kind::Loteria => 4,
    };
    let mut v: Vec<Vec<usize>> = Vec::new();
    for r in 0..n {
        v.push((0..n).map(|c| r * n + c).collect());
    }
    for c in 0..n {
        v.push((0..n).map(|r| r * n + c).collect());
    }
    v.push((0..n).map(|k| k * n + k).collect());
    v.push((0..n).map(|k| k * n + (n - 1 - k)).collect());
    if kind == Kind::Loteria {
        v.push(vec![0, 3, 12, 15]);
        v.push(vec![5, 6, 9, 10]);
    }
    v
}

impl Game for CallGame {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }

    fn to_move(&self) -> Player {
        Self::to_move_of(self.phase)
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Draw => vec![Move::Draw],
            Phase::Mark(_) => vec![Move::Mark],
            Phase::Claim(_) => vec![Move::Shout, Move::Continue],
            Phase::Over => Vec::new(),
        }
    }

    fn apply(&mut self, m: &Move) {
        match (*m, self.phase) {
            (Move::Draw, Phase::Draw) => {
                let v = self.bag.pop().expect("sac gol");
                self.drawn.push(v);
                self.claims = [false; 2];
                self.advance(Phase::Mark(Player::One));
            }
            (Move::Mark, Phase::Mark(p)) => {
                let v = self.current().expect("tragere");
                for (i, c) in self.cards[p.idx()].iter().enumerate() {
                    if *c == v {
                        self.marks[p.idx()][i] = true;
                    }
                }
                let next = match p {
                    Player::One => Phase::Mark(Player::Two),
                    Player::Two => Phase::Claim(Player::One),
                };
                self.advance(next);
            }
            (Move::Shout, Phase::Claim(p)) => {
                self.claims[p.idx()] = true;
                if p == Player::One {
                    self.advance(Phase::Claim(Player::Two));
                } else {
                    self.finish_claims();
                }
            }
            (Move::Continue, Phase::Claim(p)) => {
                if p == Player::One {
                    self.advance(Phase::Claim(Player::Two));
                } else {
                    self.finish_claims();
                }
            }
            _ => unreachable!("mutare în fază greșită"),
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.result
    }

    fn evaluate(&self, p: Player) -> i32 {
        if self.claims[p.idx()] {
            1000
        } else {
            0
        }
    }

    fn render(&self) -> String {
        let done = self.phase == Phase::Over;
        let n = self.side_len();
        let mut s = String::new();
        match self.kind {
            Kind::Bingo => s.push_str("BINGO (75 de bile)"),
            Kind::Loteria => s.push_str("LOTERÍA (54 de imagini)"),
        }
        s.push('\n');
        if let Some(v) = self.current() {
            s.push_str(&format!("Tras acum: {}\n", self.name(v)));
        } else {
            s.push_str("Nimic tras încă.\n");
        }
        let last: Vec<String> = self
            .drawn
            .iter()
            .rev()
            .take(10)
            .map(|&v| self.name(v))
            .collect();
        s.push_str(&format!(
            "Ultimele: {}\n",
            if last.is_empty() {
                "—".into()
            } else {
                last.join(", ")
            }
        ));
        s.push_str(&format!(
            "Trase: {}, rămase în sac: {}\n",
            self.drawn.len(),
            self.bag.len()
        ));
        let v = self.viewer.unwrap_or_else(|| self.to_move());
        for p in [Player::One, Player::Two] {
            if p != v && !done {
                let marked = self.marks[p.idx()].iter().filter(|&&m| m).count();
                let who = if p == Player::Two {
                    "AI-ului"
                } else {
                    "adversarului"
                };
                s.push_str(&format!("Fișa {who}: ascunsă ({marked} marcate)\n"));
                continue;
            }
            s.push_str(&format!("Fișa Jucătorului {}:\n", p.idx() + 1));
            if self.kind == Kind::Bingo {
                s.push_str("   B   I   N   G   O\n");
            }
            for r in 0..n {
                for c in 0..n {
                    let i = r * n + c;
                    let v = self.cards[p.idx()][i];
                    let mk = self.marks[p.idx()][i];
                    let txt = match self.kind {
                        Kind::Bingo => {
                            if v == 0 {
                                "LIBER".to_string()
                            } else {
                                format!("{v:>2}")
                            }
                        }
                        Kind::Loteria => format!("{:<9}", LOTERIA_PICTURES[v as usize - 1]),
                    };
                    if mk {
                        s.push_str(&format!("[{txt}]"));
                    } else {
                        s.push_str(&format!(" {txt} "));
                    }
                }
                s.push('\n');
            }
        }
        match self.phase {
            Phase::Draw => s.push_str("Mutare: «trag».\n"),
            Phase::Mark(_) => s.push_str("Ai tragerea pe fișă: «marchez».\n"),
            Phase::Claim(p) => s.push_str(&format!(
                "Jucătorul {} are o combinație: «strig» sau «continui».\n",
                p.idx() + 1
            )),
            Phase::Over => {}
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Draw => "trag",
            Move::Mark => "marchez",
            Move::Shout => "strig",
            Move::Continue => "continui",
        }
        .to_string()
    }

    fn ai_style(&self) -> AiStyle {
        // Adâncimea 0 = o singură mutare la TOATE nivelurile: la Hard/Expert adâncimea 1 ar extinde
        // răspunsurile adversarului din mâna lui reală (informație ascunsă).
        AiStyle::AlphaBeta(0)
    }
}

impl CallGame {
    fn finish_claims(&mut self) {
        self.result = match (self.claims[0], self.claims[1]) {
            (true, true) => Some(Outcome::Draw),
            (true, false) => Some(Outcome::Win(Player::One)),
            (false, true) => Some(Outcome::Win(Player::Two)),
            _ => {
                if self.bag.is_empty() {
                    Some(Outcome::Draw)
                } else {
                    None
                }
            }
        };
        self.phase = if self.result.is_some() {
            Phase::Over
        } else {
            Phase::Draw
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::GameObj;

    fn play_all(mut g: CallGame, seed: u64, shout: bool) -> CallGame {
        let mut rng = Rng::new(seed);
        let mut n = 0;
        while Game::outcome(&g).is_none() {
            let m = Game::legal_moves(&g);
            let pick = if shout {
                *m.iter().find(|x| **x == Move::Shout).unwrap_or(&m[0])
            } else {
                m[rng.below(m.len())]
            };
            g.apply(&pick);
            n += 1;
            assert!(n < 1000);
        }
        g
    }

    #[test]
    fn bingo_card_structure() {
        let g = CallGame::bingo_seed(4);
        for p in [Player::One, Player::Two] {
            let c = g.card(p);
            assert_eq!(c.len(), 25);
            assert_eq!(c[12], 0);
            for col in 0..5 {
                let mut seen = Vec::new();
                for row in 0..5 {
                    let v = c[row * 5 + col];
                    if v != 0 {
                        assert!(v > col as u8 * 15 && v <= (col as u8 + 1) * 15);
                        assert!(!seen.contains(&v));
                        seen.push(v);
                    }
                }
            }
        }
    }

    #[test]
    fn loteria_card_is_16_distinct_pictures() {
        let g = CallGame::loteria_seed(7);
        let mut c = g.card(Player::One).to_vec();
        assert_eq!(c.len(), 16);
        c.sort_unstable();
        c.dedup();
        assert_eq!(c.len(), 16);
        assert_eq!(LOTERIA_PICTURES.len(), 54);
        let mut names = LOTERIA_PICTURES.to_vec();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), 54);
    }

    #[test]
    fn marking_is_required_and_lines_are_detected() {
        let mut g = CallGame::bingo_seed(3);
        // forțăm: fișa lui J1 are ca prim număr tras ceva de pe fișă
        let v = g.card(Player::One)[0];
        g.bag.push(v);
        g.apply(&Move::Draw);
        assert_eq!(Game::to_move(&g), Player::One);
        assert_eq!(Game::legal_moves(&g), vec![Move::Mark]);
        g.apply(&Move::Mark);
        assert!(g.marks[0][0]);
    }

    #[test]
    fn shouting_only_with_a_line() {
        let mut g = CallGame::bingo_seed(5);
        for i in 0..5 {
            g.marks[0][i] = true;
        }
        assert!(g.has_line(Player::One));
        assert!(!g.has_line(Player::Two));
        let v = g.bag.pop().unwrap();
        g.drawn.push(v);
        g.advance(Phase::Mark(Player::One));
        // J1 are rând: mutările sunt strig / continui (dacă nu trebuie marcat)
        if Game::legal_moves(&g) == vec![Move::Mark] {
            g.apply(&Move::Mark);
        }
        if Game::to_move(&g) == Player::Two && Game::legal_moves(&g) == vec![Move::Mark] {
            g.apply(&Move::Mark);
        }
        let l = Game::legal_moves(&g);
        assert!(l.contains(&Move::Shout) && l.contains(&Move::Continue));
        g.apply(&Move::Shout);
        assert_eq!(Game::outcome(&g), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn loteria_corners_and_center_win() {
        let mut g = CallGame::loteria_seed(2);
        for &i in &[0usize, 3, 12, 15] {
            g.marks[0][i] = true;
        }
        assert!(g.has_line(Player::One));
        let mut h = CallGame::loteria_seed(2);
        for &i in &[5usize, 6, 9, 10] {
            h.marks[1][i] = true;
        }
        assert!(h.has_line(Player::Two));
        assert!(!h.has_line(Player::One));
    }

    #[test]
    fn games_always_end_and_ai_shouts() {
        for seed in 1..=30u64 {
            for kind in [Kind::Bingo, Kind::Loteria] {
                let g = CallGame::make(kind, seed);
                let g = play_all(g, seed, false);
                assert!(Game::outcome(&g).is_some());
                let g = CallGame::make(kind, seed);
                let g = play_all(g, seed, true);
                assert!(Game::outcome(&g).is_some());
                assert!(g.drawn_count() <= 75);
            }
        }
    }

    #[test]
    fn ai_always_shouts_when_it_has_a_line() {
        let mut g = CallGame::bingo_seed(9);
        for i in 0..5 {
            g.marks[1][i * 5] = true; // coloana B
        }
        g.phase = Phase::Claim(Player::Two);
        assert_eq!(GameObj::ai_move(&g, 1), Some("strig".to_string()));
    }

    #[test]
    fn render_hides_ai_card_until_the_end() {
        for kind in [Kind::Bingo, Kind::Loteria] {
            let mut g = CallGame::make(kind, 6);
            let mut n = 0;
            while Game::outcome(&g).is_none() && n < 12 {
                let r = Game::render(&g);
                assert!(r.contains("Fișa Jucătorului 1"));
                assert!(!r.contains("Fișa Jucătorului 2"));
                let m = Game::legal_moves(&g)[0];
                g.apply(&m);
                n += 1;
            }
        }
    }

    #[test]
    fn both_shouting_is_a_draw() {
        let mut g = CallGame::bingo_seed(11);
        for i in 0..5 {
            g.marks[0][i] = true;
            g.marks[1][i] = true;
        }
        let v = g.bag.pop().unwrap();
        g.drawn.push(v);
        g.advance(Phase::Mark(Player::One));
        let mut guard = 0;
        while Game::outcome(&g).is_none() {
            let m = Game::legal_moves(&g);
            let pick = if m.contains(&Move::Shout) {
                Move::Shout
            } else {
                m[0]
            };
            g.apply(&pick);
            guard += 1;
            assert!(guard < 10);
        }
        assert_eq!(Game::outcome(&g), Some(Outcome::Draw));
    }
}

#[cfg(test)]
mod viewer_tests {
    use super::*;

    #[test]
    fn viewer_sees_only_own_card() {
        for mut g in [CallGame::bingo_seed(3), CallGame::loteria_seed(3)] {
            Game::set_viewer(&mut g, Some(Player::One));
            let r = Game::render(&g);
            assert!(r.contains("Fișa Jucătorului 1:"), "{r}");
            assert!(!r.contains("Fișa Jucătorului 2:"), "{r}");
            assert!(r.contains("Fișa AI-ului: ascunsă"), "{r}");
            Game::set_viewer(&mut g, Some(Player::Two));
            let r = Game::render(&g);
            assert!(r.contains("Fișa Jucătorului 2:"), "{r}");
            assert!(!r.contains("Fișa Jucătorului 1:"), "{r}");
            assert!(r.contains("Fișa adversarului: ascunsă"), "{r}");
            Game::set_viewer(&mut g, None);
            let r = Game::render(&g);
            assert!(r.contains("Fișa Jucătorului 1:"), "{r}");
        }
    }
}

#[cfg(test)]
mod a3_audit {
    use super::*;

    #[test]
    fn ai_ignores_the_opponent_card_and_bag_at_every_level() {
        crate::p3_comun::ai_blind_check_with(
            "loteria",
            CallGame::loteria_seed,
            |g, me, s| {
                let h = 1 - me.idx();
                let mut r = Rng::new(s);
                g.cards[h] = make_card(g.kind, &mut r);
                let drawn = g.drawn.clone();
                g.marks[h] = g.cards[h]
                    .iter()
                    .map(|&v| v == 0 || drawn.contains(&v))
                    .collect();
                shuffle(&mut g.bag, &mut r);
            },
            3,
            1,
            crate::p3_comun::Term::Ignore,
        );
    }
}
