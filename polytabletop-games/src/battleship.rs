//! Bătălia navală (10 × 10), Jucătorul 1 (om) contra Jucătorului 2 (AI).
//!
//! * Flota: portavion (5), cuirasat (4), crucișător (3), submarin (3),
//!   distrugător (2). Navele sunt orizontale sau verticale și nu se suprapun
//!   (se pot atinge).
//! * Plasare ascunsă: Jucătorul 1 își așază navele pe rând (mutare `a1h` =
//!   colțul stânga-sus în a1, nava se întinde spre dreapta; `a1v` = în jos) sau
//!   scrie `aleator`. Flota AI-ului se așază printr-o mutare forțată separată
//!   (`plasez`), cu `Rng`-ul jocului, și nu apare în `render()` până la final.
//! * Tragere pe rând, câte o lovitură (`e5`); rezultat: apă / lovit / scufundat.
//! * Când ultima navă a cuiva e scufundată, cel învins confirmă printr-o mutare
//!   forțată (`recunosc`), ca AI-ul să nu „vadă” loviturile câștigătoare.
//!
//! AI fără trișat: vânătoare–țintire. Pentru fiecare lovitură posibilă,
//! scorul se calculează ÎNAINTE de a o aplica, doar din tabla ei de lovituri
//! (apă / lovit / scufundat) și din lungimile navelor încă nescufundate:
//! hartă de densitate a poziționărilor posibile, cu pondere mare pentru cele
//! care acoperă lovituri nescufundate încă. `evaluate` citește doar acest scor.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const N: usize = 10;
pub const FLEET: [usize; 5] = [5, 4, 3, 3, 2];
pub const SHIP_NAMES: [&str; 5] = [
    "portavion",
    "cuirasat",
    "crucișător",
    "submarin",
    "distrugător",
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Place(usize, bool), // celulă (r*10+c), orizontal?
    AutoPlace,
    AiPlace,
    Shot(usize),
    Concede,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    PlaceOne(usize),
    PlaceTwo,
    Fire,
    Concede,
    Over,
}

#[derive(Clone)]
pub struct Battleship {
    phase: Phase,
    /// `owner[p][celulă]` = indexul navii lui `p` (sau 255).
    owner: [[u8; N * N]; 2],
    hits: [[u8; 5]; 2],
    /// `shots[p][celulă]` = loviturile trase DE `p` în tabla adversă:
    /// 0 necunoscut, 1 apă, 2 lovit, 3 scufundat.
    shots: [[u8; N * N]; 2],
    side: Player,
    rng: Rng,
    hint: i32,
    hint_for: Player,
    winner: Option<Player>,
    log: String,
    /// `Some(p)` = render shows `p`'s fleet; `None` = hot-seat (the mover's).
    viewer: Option<Player>,
}

impl Default for Battleship {
    fn default() -> Self {
        Self::new()
    }
}

fn cell_name(c: usize) -> String {
    format!("{}{}", (b'a' + (c % N) as u8) as char, c / N + 1)
}

fn ship_cells(start: usize, len: usize, horiz: bool) -> Option<Vec<usize>> {
    let (r, c) = (start / N, start % N);
    if horiz {
        if c + len > N {
            return None;
        }
        Some((0..len).map(|k| r * N + c + k).collect())
    } else {
        if r + len > N {
            return None;
        }
        Some((0..len).map(|k| (r + k) * N + c).collect())
    }
}

impl Battleship {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Battleship {
            phase: Phase::PlaceOne(0),
            owner: [[255; N * N]; 2],
            hits: [[0; 5]; 2],
            shots: [[0; N * N]; 2],
            side: Player::One,
            rng: Rng::new(seed),
            hint: 0,
            hint_for: Player::One,
            winner: None,
            log: String::new(),
            viewer: Some(Player::One),
        }
    }

    /// Plasează aleator flota lui `p` (folosit pentru AI și pentru `aleator`).
    pub fn random_fleet(&mut self, p: Player) {
        self.owner[p.idx()] = [255; N * N];
        for (i, &len) in FLEET.iter().enumerate() {
            loop {
                let start = self.rng.below(N * N);
                let horiz = self.rng.below(2) == 0;
                if let Some(cells) = ship_cells(start, len, horiz) {
                    if cells.iter().all(|&c| self.owner[p.idx()][c] == 255) {
                        for c in cells {
                            self.owner[p.idx()][c] = i as u8;
                        }
                        break;
                    }
                }
            }
        }
    }

    pub fn ships_left(&self, p: Player) -> usize {
        (0..5)
            .filter(|&i| (self.hits[p.idx()][i] as usize) < FLEET[i])
            .count()
    }

    /// Câte lovituri a tras `p` până acum.
    pub fn shots_fired(&self, p: Player) -> usize {
        self.shots[p.idx()].iter().filter(|&&x| x != 0).count()
    }

    pub fn in_fire_phase(&self) -> bool {
        matches!(self.phase, Phase::Fire)
    }

    fn sunk(&self, owner: Player, i: usize) -> bool {
        self.hits[owner.idx()][i] as usize >= FLEET[i]
    }

    /// Harta de densitate a lui `shooter`: doar informația din tabla lui de lovituri.
    fn density(&self, shooter: Player) -> [i32; N * N] {
        let sh = &self.shots[shooter.idx()];
        let foe = shooter.other();
        let mut d = [0i32; N * N];
        for (i, &len) in FLEET.iter().enumerate() {
            if self.sunk(foe, i) {
                continue;
            }
            for start in 0..N * N {
                for horiz in [true, false] {
                    let Some(cells) = ship_cells(start, len, horiz) else {
                        continue;
                    };
                    if cells.iter().any(|&c| sh[c] == 1 || sh[c] == 3) {
                        continue;
                    }
                    let k = cells.iter().filter(|&&c| sh[c] == 2).count() as i32;
                    let w = 1 + 40 * k * k;
                    for &c in &cells {
                        if sh[c] == 0 {
                            d[c] += w;
                        }
                    }
                }
            }
        }
        d
    }

    fn fire(&mut self, shooter: Player, c: usize) {
        let foe = shooter.other();
        let o = self.owner[foe.idx()][c];
        if o == 255 {
            self.shots[shooter.idx()][c] = 1;
            self.log = format!(
                "Jucătorul {} trage în {}: apă.",
                shooter.idx() + 1,
                cell_name(c)
            );
            return;
        }
        let i = o as usize;
        self.hits[foe.idx()][i] += 1;
        if self.hits[foe.idx()][i] as usize == FLEET[i] {
            for cc in 0..N * N {
                if self.owner[foe.idx()][cc] == o {
                    self.shots[shooter.idx()][cc] = 3;
                }
            }
            self.log = format!(
                "Jucătorul {} trage în {}: scufundat ({}, {}).",
                shooter.idx() + 1,
                cell_name(c),
                SHIP_NAMES[i],
                FLEET[i]
            );
        } else {
            self.shots[shooter.idx()][c] = 2;
            self.log = format!(
                "Jucătorul {} trage în {}: lovit.",
                shooter.idx() + 1,
                cell_name(c)
            );
        }
    }

    /// Flota în clar (pentru teste / afișare la final).
    pub fn fleet_cells(&self, p: Player) -> Vec<usize> {
        (0..N * N)
            .filter(|&c| self.owner[p.idx()][c] != 255)
            .collect()
    }

    /// Înlocuiește flota lui `p` (pentru teste de secret).
    pub fn set_fleet(&mut self, p: Player, seed: u64) {
        let saved = self.rng.clone();
        self.rng = Rng::new(seed);
        self.random_fleet(p);
        self.rng = saved;
    }
}

impl Game for Battleship {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::PlaceOne(i) => {
                let mut v = Vec::new();
                for start in 0..N * N {
                    for horiz in [true, false] {
                        if let Some(cells) = ship_cells(start, FLEET[i], horiz) {
                            if cells.iter().all(|&c| self.owner[0][c] == 255) {
                                v.push(Move::Place(start, horiz));
                            }
                        }
                    }
                }
                if i == 0 {
                    v.push(Move::AutoPlace);
                }
                v
            }
            Phase::PlaceTwo => vec![Move::AiPlace],
            Phase::Fire => (0..N * N)
                .filter(|&c| self.shots[self.side.idx()][c] == 0)
                .map(Move::Shot)
                .collect(),
            Phase::Concede => vec![Move::Concede],
            Phase::Over => Vec::new(),
        }
    }

    fn apply(&mut self, m: &Move) {
        match (*m, self.phase) {
            (Move::Place(start, horiz), Phase::PlaceOne(i)) => {
                for c in ship_cells(start, FLEET[i], horiz).expect("plasare") {
                    self.owner[0][c] = i as u8;
                }
                if i + 1 == FLEET.len() {
                    self.phase = Phase::PlaceTwo;
                    self.side = Player::Two;
                } else {
                    self.phase = Phase::PlaceOne(i + 1);
                }
            }
            (Move::AutoPlace, Phase::PlaceOne(_)) => {
                self.random_fleet(Player::One);
                self.phase = Phase::PlaceTwo;
                self.side = Player::Two;
            }
            (Move::AiPlace, Phase::PlaceTwo) => {
                self.random_fleet(Player::Two);
                self.phase = Phase::Fire;
                self.side = Player::One;
                self.log = "Flotele sunt așezate. Jucătorul 1 trage primul.".into();
            }
            (Move::Shot(c), Phase::Fire) => {
                let me = self.side;
                // scorul se ia din informația de dinainte de lovitură
                self.hint = self.density(me)[c];
                self.hint_for = me;
                self.fire(me, c);
                if self.ships_left(me.other()) == 0 {
                    self.phase = Phase::Concede;
                    self.winner = Some(me);
                }
                self.side = me.other();
            }
            (Move::Concede, Phase::Concede) => {
                self.phase = Phase::Over;
            }
            _ => unreachable!("mutare în fază greșită"),
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase == Phase::Over {
            self.winner.map(Outcome::Win)
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        if self.hint_for == p {
            self.hint
        } else {
            0
        }
    }

    fn render(&self) -> String {
        let reveal = self.phase == Phase::Over || self.phase == Phase::Concede;
        let mut s = String::new();
        if !self.log.is_empty() {
            s.push_str(&self.log);
            s.push('\n');
        }
        // tabla proprie a privitorului și tabla lui de lovituri în flota adversă
        let v = self.viewer.unwrap_or(self.side);
        let (vi, oi) = (v.idx(), v.other().idx());
        s.push_str(&format!(
            "   Flota ta (J{})              Loviturile tale în flota J{}\n",
            vi + 1,
            oi + 1
        ));
        s.push_str("   a b c d e f g h i j        a b c d e f g h i j\n");
        for r in 0..N {
            s.push_str(&format!("{:>2} ", r + 1));
            for c in 0..N {
                let cell = r * N + c;
                let o = self.owner[vi][cell];
                let hit = self.shots[oi][cell];
                let ch = if o == 255 {
                    if hit == 1 {
                        'o'
                    } else {
                        '.'
                    }
                } else if self.sunk(v, o as usize) {
                    '#'
                } else if hit == 2 {
                    'X'
                } else {
                    'S'
                };
                s.push(ch);
                s.push(' ');
            }
            s.push_str(&format!("   {:>2} ", r + 1));
            for c in 0..N {
                let cell = r * N + c;
                let sh = self.shots[vi][cell];
                let ch = match sh {
                    1 => 'o',
                    2 => 'X',
                    3 => '#',
                    _ => {
                        if reveal && self.owner[oi][cell] != 255 {
                            's'
                        } else {
                            '.'
                        }
                    }
                };
                s.push(ch);
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str("S = navă proprie, X = lovit, # = scufundat, o = apă");
        if reveal {
            s.push_str(", s = navă adversă nedescoperită");
        }
        s.push('\n');
        match self.phase {
            Phase::PlaceOne(i) => s.push_str(&format!(
                "Așază {} ({} celule): ex. a1h (orizontal) sau a1v (vertical); sau «aleator».\n",
                SHIP_NAMES[i], FLEET[i]
            )),
            Phase::PlaceTwo => s.push_str("Urmează așezarea flotei AI-ului: mutare «plasez».\n"),
            Phase::Fire => s.push_str(&format!(
                "Nave rămase: tu {}, adversar {}. Trage: ex. e5.\n",
                self.ships_left(v),
                self.ships_left(v.other())
            )),
            Phase::Concede => {
                s.push_str("Toate navele unui jucător sunt scufundate: «recunosc».\n")
            }
            Phase::Over => {}
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Place(c, h) => format!("{}{}", cell_name(*c), if *h { "h" } else { "v" }),
            Move::AutoPlace => "aleator".into(),
            Move::AiPlace => "plasez".into(),
            Move::Shot(c) => cell_name(*c),
            Move::Concede => "recunosc".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::GameObj;

    fn play(g: &mut Battleship, m: &str) -> Result<String, String> {
        GameObj::play(g, m)
    }
    fn ai(g: &Battleship, seed: u64) -> Option<String> {
        GameObj::ai_move(g, seed)
    }
    fn out(g: &Battleship) -> Option<Outcome> {
        Game::outcome(g)
    }
    fn side(g: &Battleship) -> Player {
        Game::to_move(g)
    }
    fn txt(g: &Battleship) -> String {
        Game::render(g)
    }

    fn started(seed: u64) -> Battleship {
        let mut g = Battleship::with_seed(seed);
        g.apply(&Move::AutoPlace);
        g.apply(&Move::AiPlace);
        g
    }

    #[test]
    fn fleet_has_17_cells_and_no_overlap() {
        let g = started(3);
        assert_eq!(g.fleet_cells(Player::One).len(), 17);
        assert_eq!(g.fleet_cells(Player::Two).len(), 17);
    }

    #[test]
    fn manual_placement_flow() {
        let mut g = Battleship::with_seed(1);
        let names = ["a1h", "a2h", "a3h", "a4h", "a5h"];
        for n in names {
            assert_eq!(play(&mut g, n).unwrap(), n);
        }
        assert_eq!(side(&g), Player::Two);
        assert_eq!(Game::legal_moves(&g), vec![Move::AiPlace]);
        // nu poți suprapune
        let mut h = Battleship::with_seed(1);
        play(&mut h, "a1h").unwrap();
        assert!(play(&mut h, "a1v").is_err());
        assert!(play(&mut h, "h1h").is_err()); // 4 celule nu încap de la h
    }

    #[test]
    fn hits_and_sinking_are_reported() {
        let mut g = Battleship::with_seed(9);
        for n in ["a1h", "a2h", "a3h", "a4h", "a5h"] {
            play(&mut g, n).unwrap();
        }
        play(&mut g, "plasez").unwrap();
        let ship = g.fleet_cells(Player::Two);
        let first = ship[0];
        g.apply(&Move::Shot(first));
        assert!(txt(&g).contains("lovit") || txt(&g).contains("scufundat"));
    }

    #[test]
    fn full_game_ends_with_forced_concede() {
        let mut g = started(5);
        let mut rng = Rng::new(2);
        let mut n = 0;
        while out(&g).is_none() {
            let m = Game::legal_moves(&g);
            g.apply(&m[rng.below(m.len())]);
            n += 1;
            assert!(n < 400);
        }
        assert!(out(&g).is_some());
    }

    #[test]
    fn ai_choice_ignores_the_hidden_fleet() {
        for seed in 1..=12u64 {
            let a = started(seed);
            let mut b = a.clone();
            b.set_fleet(Player::One, seed * 7919 + 13); // AI-ul trage în flota lui J1
            assert_ne!(a.fleet_cells(Player::One), b.fleet_cells(Player::One));
            // J1 trage ceva ca să ajungă rândul AI-ului pe tablă identică (apă la a1 în ambele?)
            let mut a1 = a.clone();
            let mut b1 = b.clone();
            // forțăm J1 să tragă în celule goale în ambele flote
            let free: Vec<usize> = (0..N * N)
                .filter(|&c| a.owner[1][c] == 255)
                .take(1)
                .collect();
            a1.apply(&Move::Shot(free[0]));
            b1.apply(&Move::Shot(free[0]));
            assert_eq!(
                ai(&a1, seed),
                ai(&b1, seed),
                "AI-ul a depins de flota ascunsă"
            );
        }
    }

    #[test]
    fn ai_sinks_a_fleet_much_faster_than_blind_random() {
        let mut total = 0usize;
        for seed in 1..=10u64 {
            let mut g = Battleship::with_seed(seed);
            g.apply(&Move::AutoPlace);
            g.apply(&Move::AiPlace);
            let mut turns = 0;
            // J1 trage mereu în ordine; AI-ul lui J2 vânează
            while out(&g).is_none() {
                let m = if side(&g) == Player::Two {
                    let s = ai(&g, seed * 31 + turns).unwrap();
                    Game::legal_moves(&g)
                        .into_iter()
                        .find(|m| g.format_move(m) == s)
                        .unwrap()
                } else {
                    Game::legal_moves(&g)[0]
                };
                g.apply(&m);
                turns += 1;
                assert!(turns < 600);
            }
            total += g.shots_fired(Player::Two);
        }
        let avg = total / 10;
        assert!(avg < 75, "AI-ul a avut nevoie în medie de {avg} lovituri");
    }
}

#[cfg(test)]
mod viewer_tests {
    use super::*;

    fn started(seed: u64) -> Battleship {
        let mut g = Battleship::with_seed(seed);
        g.apply(&Move::AutoPlace);
        g.apply(&Move::AiPlace);
        g
    }

    /// Cells of the left (own-fleet) board that the render marks as ships.
    fn own_ship_cells(r: &str) -> Vec<usize> {
        let mut v = Vec::new();
        for line in r.lines().filter(|l| {
            l.split_whitespace()
                .next()
                .is_some_and(|t| t.parse::<usize>().is_ok())
        }) {
            let row: usize = line.split_whitespace().next().unwrap().parse().unwrap();
            let b = line.as_bytes();
            for c in 0..N {
                if b[3 + 2 * c] == b'S' {
                    v.push((row - 1) * N + c);
                }
            }
        }
        v
    }

    fn cells(g: &Battleship, p: Player) -> Vec<usize> {
        (0..N * N).filter(|&c| g.owner[p.idx()][c] != 255).collect()
    }

    #[test]
    fn each_viewer_sees_only_own_fleet() {
        let mut g = started(3);
        Game::set_viewer(&mut g, Some(Player::One));
        assert_eq!(own_ship_cells(&Game::render(&g)), cells(&g, Player::One));
        Game::set_viewer(&mut g, Some(Player::Two));
        let r = Game::render(&g);
        assert_eq!(own_ship_cells(&r), cells(&g, Player::Two));
        assert!(r.contains("Flota ta (J2)"), "{r}");
        assert_ne!(cells(&g, Player::One), cells(&g, Player::Two));
        // niciun 's' (navă adversă nedescoperită) cât timp jocul continuă
        assert!(!r.contains(", s = navă adversă"));
    }

    #[test]
    fn hotseat_follows_the_mover() {
        let mut g = started(3);
        Game::set_viewer(&mut g, None);
        assert_eq!(own_ship_cells(&Game::render(&g)), cells(&g, Player::One));
        let m = Game::legal_moves(&g)[0];
        g.apply(&m);
        assert_eq!(Game::to_move(&g), Player::Two);
        assert_eq!(own_ship_cells(&Game::render(&g)), cells(&g, Player::Two));
    }
}
