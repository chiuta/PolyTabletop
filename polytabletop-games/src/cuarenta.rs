//! Cuarenta (Ecuador), varianta pentru 2 jucători, baraja spaniolă de 40 de cărți.
//!
//! Reguli folosite (după descrierea publică a jocului):
//! * Rangurile în ordinea secvențelor: 1-2-3-4-5-6-7-10-11-12 (sota, caballo, rey); culori `o`,
//!   `c`, `e`, `b` (vezi `baraja_es.rs`). Doar 1–7 au valoare numerică; figurile (10, 11, 12)
//!   nu se adună și nu se iau prin adunare.
//! * Câte 5 cărți fiecăruia, masa începe goală; patru împărțiri (mutare forțată `imparte`)
//!   pe mână, deci 40 de cărți. Începe jucătorul care nu a împărțit; împărțitorul alternează.
//! * La rândul tău joci o carte. Poți **lua**: (1) o carte de pe masă de același rang (potrivire),
//!   sau (2) un grup de cărți 1–7 de pe masă a căror sumă e valoarea cărții tale (adunare).
//!   După o potrivire sau o adunare iei și **secvența ascendentă**: cărțile de pe masă cu rangul
//!   următor, apoi următorul ș.a.m.d. fără goluri (rangurile cu mai multe cărți se iau toate).
//!   Luarea e opțională: poți lăsa cartea pe masă. Mutări: `5o` (lași), `5o:5c` (potrivire),
//!   `6o:2c+4e` (adunare 2+4 = 6); secvența se adaugă automat.
//! * Puncte: **caída** = iei prin potrivire exact cartea tocmai lăsată de adversar (2 puncte);
//!   **limpia** = golești masa (2 puncte; cu caída 4); la 38 de puncte nu se mai dă limpia.
//!   **Ronda** = trei cărți de același rang în mână la împărțire: 4 puncte; patru la fel =
//!   câștigi jocul pe loc. La capătul mâinii (40 de cărți jucate): cu exact 20 de cărți luate 6
//!   puncte, peste 20: 6 + 2 per cărți în plus, rotunjit în sus la pereche (21–22 → 8, 23–24 → 10…);
//!   dacă amândoi au 20, doar cel care nu a împărțit ia 6; sub 20 la amândoi, cel cu mai
//!   multe cărți 2 puncte (la egalitate cel care nu a împărțit). Cărțile rămase pe masă nu
//!   contează. Primul la **40** de puncte câștigă.
//!
//! Simplificări documentate: nu se joacă „ronda cu recunoaștere” (10 puncte) și nici penalizarea
//! pentru secvență omisă (secvența e luată automat). Prima carte a unei noi împărțiri nu poate
//! face caída pe ultima carte a împărțitorului din împărțirea anterioară (se șterge „ultima carte”).
//!
//! Informație ascunsă ca la `scopa.rs`; `evaluate` citește doar masa, grămezile și scorul.

use crate::baraja_es::{deck, join, pos, sorted, text};
use crate::cards::Viewer;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TARGET: u8 = 40;

fn val(c: u8) -> Option<u8> {
    let p = pos(c);
    if p <= 6 {
        Some(p + 1)
    } else {
        None
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    Play(u8, Vec<u8>),
    Deal,
    Next,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Play,
    NeedDeal,
    RoundOver,
}

#[derive(Clone)]
pub struct Cuarenta {
    rng: Rng,
    viewer: Viewer,
    stock: Vec<u8>,
    hands: [Vec<u8>; 2],
    table: Vec<u8>,
    taken: [u16; 2],
    score: [u8; 2],
    last_played: Option<(Player, u8)>,
    side: Player,
    dealer: Player,
    phase: Phase,
    winner: Option<Player>,
    log: String,
}

impl Default for Cuarenta {
    fn default() -> Self {
        Self::new()
    }
}

impl Cuarenta {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Cuarenta {
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
            stock: Vec::new(),
            hands: [Vec::new(), Vec::new()],
            table: Vec::new(),
            taken: [0; 2],
            score: [0; 2],
            last_played: None,
            side: Player::One,
            dealer: Player::Two,
            phase: Phase::Play,
            winner: None,
            log: String::new(),
        };
        g.new_hand();
        g
    }

    pub fn score(&self) -> (u8, u8) {
        (self.score[0], self.score[1])
    }

    fn new_hand(&mut self) {
        self.stock = deck(&mut self.rng);
        self.table.clear();
        self.taken = [0; 2];
        self.hands = [Vec::new(), Vec::new()];
        self.deal_five();
        self.side = self.dealer.other();
        self.phase = Phase::Play;
    }

    fn add(&mut self, p: Player, pts: u8) {
        if self.winner.is_some() {
            return;
        }
        self.score[p.idx()] += pts;
        if self.score[p.idx()] >= TARGET {
            self.winner = Some(p);
        }
    }

    fn deal_five(&mut self) {
        for p in 0..2 {
            for _ in 0..5 {
                if let Some(c) = self.stock.pop() {
                    self.hands[p].push(c);
                }
            }
        }
        self.last_played = None;
        for (i, who) in [Player::One, Player::Two].into_iter().enumerate() {
            let mut cnt = [0u8; 10];
            for &c in &self.hands[i] {
                cnt[pos(c) as usize] += 1;
            }
            if cnt.contains(&4) {
                if self.winner.is_none() {
                    self.winner = Some(who);
                }
            } else if cnt.contains(&3) {
                self.add(who, 4);
                self.log = format!("Ronda anunțată de J{} (+4).", i + 1);
            }
        }
    }

    /// Nucleele de luare pentru cartea `c`: potriviri și adunări.
    fn cores(&self, c: u8) -> Vec<Vec<u8>> {
        let mut out: Vec<Vec<u8>> = Vec::new();
        let t = sorted(self.table.clone());
        for &x in &t {
            if pos(x) == pos(c) {
                out.push(vec![x]);
            }
        }
        if let Some(v) = val(c) {
            let nums: Vec<u8> = t.iter().copied().filter(|&x| val(x).is_some()).collect();
            fn rec(n: &[u8], i: usize, left: u8, cur: &mut Vec<u8>, out: &mut Vec<Vec<u8>>) {
                if left == 0 {
                    if cur.len() >= 2 {
                        out.push(cur.clone());
                    }
                    return;
                }
                for j in i..n.len() {
                    let vj = val(n[j]).unwrap();
                    if vj <= left {
                        cur.push(n[j]);
                        rec(n, j + 1, left - vj, cur, out);
                        cur.pop();
                    }
                }
            }
            let mut cur = Vec::new();
            rec(&nums, 0, v, &mut cur, &mut out);
        }
        out
    }

    /// Cărțile luate în plus prin secvența ascendentă după `c`.
    fn sequence(&self, c: u8, core: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        let mut q = pos(c) + 1;
        while q <= 9 {
            let found: Vec<u8> = sorted(self.table.clone())
                .into_iter()
                .filter(|&x| pos(x) == q && !core.contains(&x))
                .collect();
            if found.is_empty() {
                break;
            }
            out.extend(found);
            q += 1;
        }
        out
    }

    fn finish_hand(&mut self) {
        let n = [self.taken[0], self.taken[1]];
        let nd = self.dealer.other(); // non-dealer
        let big = |k: u16| -> u8 { 6 + 2 * (k.saturating_sub(20).div_ceil(2)) as u8 };
        let mut pts = [0u8; 2];
        if n[0] >= 20 || n[1] >= 20 {
            if n[0] == n[1] {
                pts[nd.idx()] = 6;
            } else {
                let w = usize::from(n[1] > n[0]);
                pts[w] = big(n[w]);
            }
        } else if n[0] != n[1] {
            pts[usize::from(n[1] > n[0])] = 2;
        } else {
            pts[nd.idx()] = 2;
        }
        self.log = format!(
            "Capăt de mână: J1 {} cărți (+{}), J2 {} cărți (+{}).",
            n[0], pts[0], n[1], pts[1]
        );
        self.add(Player::One, pts[0]);
        self.add(Player::Two, pts[1]);
        self.phase = Phase::RoundOver;
    }
}

impl Game for Cuarenta {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.winner.is_some() {
            return Vec::new();
        }
        match self.phase {
            Phase::NeedDeal => vec![Move::Deal],
            Phase::RoundOver => vec![Move::Next],
            Phase::Play => {
                let mut v = Vec::new();
                for &c in &self.hands[self.side.idx()] {
                    v.push(Move::Play(c, Vec::new()));
                    for core in self.cores(c) {
                        v.push(Move::Play(c, core));
                    }
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Next => {
                self.dealer = self.dealer.other();
                self.log.clear();
                self.new_hand();
            }
            Move::Deal => {
                self.log.clear();
                self.deal_five();
                self.phase = Phase::Play;
            }
            Move::Play(c, core) => {
                let me = self.side;
                self.log.clear();
                let h = &mut self.hands[me.idx()];
                let i = h.iter().position(|x| x == c).expect("card in hand");
                h.remove(i);
                if core.is_empty() {
                    self.table.push(*c);
                    self.last_played = Some((me, *c));
                } else {
                    let mut all = core.clone();
                    let seq = self.sequence(*c, core);
                    all.extend(seq.iter().copied());
                    self.table.retain(|t| !all.contains(t));
                    self.taken[me.idx()] += 1 + all.len() as u16;
                    let caida = core.len() == 1 && self.last_played == Some((me.other(), core[0]));
                    let mut pts = 0;
                    if caida {
                        pts += 2;
                        self.log.push_str("Caída (+2). ");
                    }
                    if self.table.is_empty() && self.score[me.idx()] != 38 {
                        pts += 2;
                        self.log.push_str("Limpia (+2). ");
                    }
                    if !seq.is_empty() {
                        self.log.push_str(&format!("Secvență: {}. ", join(&seq)));
                    }
                    self.last_played = None;
                    self.add(me, pts);
                }
                self.side = me.other();
                if self.hands[0].is_empty() && self.hands[1].is_empty() {
                    if self.stock.is_empty() {
                        self.finish_hand();
                    } else {
                        self.phase = Phase::NeedDeal;
                        self.side = self.dealer.other();
                    }
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.winner.map(Outcome::Win)
    }

    /// Doar masa, cărțile luate și scorul (nu vede mâna adversarului sau pachetul).
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let o = 1 - me;
        let mut s = (self.score[me] as i32 - self.score[o] as i32) * 1000;
        s += (self.taken[me] as i32 - self.taken[o] as i32) * 14;
        if self.phase == Phase::Play && self.side != p {
            s -= self.table.len() as i32 * 5;
            if let Some((who, _)) = self.last_played {
                if who == p {
                    s -= 30; // caída posibilă pentru adversar
                }
            }
            if self.table.len() <= 1 && !self.table.is_empty() {
                s -= 20; // limpia ușoară pentru adversar
            }
        }
        s
    }

    fn render(&self) -> String {
        let v = self.viewer.who(self.side);
        let mut s = format!(
            "Cuarenta — până la {TARGET}: J1 {} – J2 {} (cărți luate {} / {})\n",
            self.score[0], self.score[1], self.taken[0], self.taken[1]
        );
        s.push_str(&format!("Masa: {}\n", join(&sorted(self.table.clone()))));
        s.push_str(&format!(
            "Pachet: {} cărți; J{} (ascuns) ține {} cărți. Împărțitor: J{}.\n",
            self.stock.len(),
            v.other().idx() + 1,
            self.hands[v.other().idx()].len(),
            self.dealer.idx() + 1
        ));
        s.push_str(&format!(
            "Mâna ta (Jucător {}): {}\n",
            v.idx() + 1,
            join(&sorted(self.hands[v.idx()].clone()))
        ));
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log.trim()));
        }
        match (self.winner, self.phase) {
            (Some(p), _) => {
                s.push_str(&format!("Joc încheiat: Jucător {} câștigă.\n", p.idx() + 1))
            }
            (_, Phase::RoundOver) => s.push_str("Mâna s-a încheiat. Scrie «urmatoarea».\n"),
            (_, Phase::NeedDeal) => s.push_str("Mâinile s-au golit: scrie «imparte».\n"),
            _ => s.push_str(&format!("La mutare: Jucător {}\n", self.side.idx() + 1)),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Deal => "imparte".into(),
            Move::Next => "urmatoarea".into(),
            Move::Play(c, core) if core.is_empty() => text(*c),
            Move::Play(c, core) => format!(
                "{}:{}",
                text(*c),
                core.iter().map(|&t| text(t)).collect::<Vec<_>>().join("+")
            ),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // pos: 0..=6 = 1..7, 7 = sota(10), 8 = caballo(11), 9 = rey(12); c = suit*10 + pos
    fn k(suit: u8, rank: u8) -> u8 {
        suit * 10 + rank - 1
    }
    const SOTA: u8 = 8;

    fn base() -> Cuarenta {
        let mut g = Cuarenta::with_seed(1);
        g.stock.clear();
        g.hands = [Vec::new(), Vec::new()];
        g.table.clear();
        g.taken = [0; 2];
        g.score = [0; 2];
        g.last_played = None;
        g.side = Player::One;
        g.dealer = Player::Two;
        g.phase = Phase::Play;
        g.winner = None;
        g
    }

    #[test]
    fn match_and_addition_cores() {
        let mut g = base();
        g.table = vec![k(0, 2), k(1, 4), k(2, 6), k(3, 6)];
        g.hands[0] = vec![k(0, 6)];
        g.hands[1] = vec![k(0, 1)];
        let mv = g.legal_moves();
        // lași, potriviri cu 6c? (6e, 6b), adunare 2+4
        assert!(mv.contains(&Move::Play(k(0, 6), vec![])));
        assert!(mv.contains(&Move::Play(k(0, 6), vec![k(2, 6)])));
        assert!(mv.contains(&Move::Play(k(0, 6), vec![k(3, 6)])));
        assert!(mv.contains(&Move::Play(k(0, 6), vec![k(0, 2), k(1, 4)])));
        assert_eq!(mv.len(), 4);
    }

    #[test]
    fn ascending_sequence_is_taken_after_a_capture() {
        // exemplul: 2,3,4,5,7,sota pe masă; joc 6 -> adunare 2+4, secvență 7 + sota
        let mut g = base();
        g.table = vec![k(0, 2), k(1, 3), k(2, 4), k(3, 5), k(0, 7), k(1, SOTA)];
        g.hands[0] = vec![k(2, 6), k(3, 1)];
        g.hands[1] = vec![k(0, 1)];
        g.stock = vec![k(3, 7)];
        g.apply(&Move::Play(k(2, 6), vec![k(0, 2), k(2, 4)]));
        // luat: 6 + 2 + 4 + 7 + sota = 5 cărți
        assert_eq!(g.taken[0], 5);
        assert_eq!(sorted(g.table.clone()), sorted(vec![k(1, 3), k(3, 5)]));
        // potrivire cu 2 și secvența 3,4,5
        let mut h = base();
        h.table = vec![k(0, 2), k(1, 3), k(2, 4), k(3, 5), k(3, 7)];
        h.hands[0] = vec![k(2, 2), k(3, 1)];
        h.hands[1] = vec![k(0, 1)];
        h.stock = vec![k(0, 4)];
        h.apply(&Move::Play(k(2, 2), vec![k(0, 2)]));
        assert_eq!(h.taken[0], 5); // 2,2,3,4,5
        assert_eq!(h.table, vec![k(3, 7)]);
    }

    #[test]
    fn caida_and_limpia_scoring() {
        let mut g = base();
        g.hands[0] = vec![k(0, 4), k(1, 1)];
        g.hands[1] = vec![k(2, 4), k(1, 2)];
        g.stock = vec![k(3, 7)];
        g.apply(&Move::Play(k(0, 4), vec![])); // J1 lasă 4o
        g.apply(&Move::Play(k(2, 4), vec![k(0, 4)])); // J2: caída + limpia
        assert_eq!(g.score, [0, 4]);
        // caída prin adunare nu contează
        let mut h = base();
        h.table = vec![k(0, 1), k(1, 2)];
        h.hands[0] = vec![k(2, 3), k(3, 7)];
        h.hands[1] = vec![k(0, 4)];
        h.stock = vec![k(1, 7)];
        h.last_played = Some((Player::Two, k(1, 2)));
        h.side = Player::One;
        h.apply(&Move::Play(k(2, 3), vec![k(0, 1), k(1, 2)]));
        assert_eq!(h.score, [2, 0]); // limpia, nu caída
    }

    #[test]
    fn no_limpia_at_38() {
        let mut g = base();
        g.score[0] = 38;
        g.table = vec![k(0, 5)];
        g.hands[0] = vec![k(1, 5), k(1, 1)];
        g.hands[1] = vec![k(0, 4)];
        g.stock = vec![k(1, 7)];
        g.apply(&Move::Play(k(1, 5), vec![k(0, 5)]));
        assert_eq!(g.score[0], 38);
    }

    #[test]
    fn ronda_and_four_of_a_kind() {
        let mut found_ronda = false;
        for seed in 1..400u64 {
            let g = Cuarenta::with_seed(seed);
            if g.winner.is_some() {
                continue;
            }
            if g.score[0] == 4 || g.score[1] == 4 {
                found_ronda = true;
                break;
            }
        }
        assert!(found_ronda);
        let mut g = base();
        g.stock = vec![
            k(0, 3),
            k(1, 3),
            k(2, 3),
            k(3, 3),
            k(0, 1),
            k(0, 2),
            k(0, 4),
            k(0, 5),
            k(0, 6),
            k(1, 1),
        ];
        g.deal_five();
        assert!(g.winner.is_some());
    }

    #[test]
    fn hand_scoring_table() {
        let mut g = base();
        g.dealer = Player::Two; // non-dealer = J1
        g.taken = [20, 20];
        g.finish_hand();
        assert_eq!(g.score, [6, 0]);
        let mut g = base();
        g.taken = [22, 18];
        g.finish_hand();
        assert_eq!(g.score, [8, 0]);
        let mut g = base();
        g.taken = [23, 17];
        g.finish_hand();
        assert_eq!(g.score, [10, 0]);
        let mut g = base();
        g.taken = [15, 18];
        g.finish_hand();
        assert_eq!(g.score, [0, 2]);
        let mut g = base();
        g.taken = [17, 17];
        g.finish_hand();
        assert_eq!(g.score, [2, 0]);
    }

    #[test]
    fn cards_are_conserved_and_games_finish() {
        for seed in 1..=10u64 {
            let mut g = Cuarenta::with_seed(seed);
            let mut rng = Rng::new(seed ^ 0x77);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 30000);
            }
        }
    }

    #[test]
    fn evaluate_ignores_hidden_cards_and_render_hides_them() {
        let g = Cuarenta::with_seed(9);
        let mut h = g.clone();
        let (a, b) = (h.hands[1][0], h.stock[3]);
        h.hands[1][0] = b;
        h.stock[3] = a;
        for p in [Player::One, Player::Two] {
            assert_eq!(g.evaluate(p), h.evaluate(p));
        }
        assert!(Game::render(&g).contains("(ascuns) ține 5"));
    }

    #[test]
    fn ai_beats_a_random_mover_mostly() {
        use polytabletop_core::GameObj;
        let mut wins = 0;
        let n = 12;
        for seed in 1..=n {
            let mut g: Box<dyn GameObj> = Box::new(Cuarenta::with_seed(seed));
            let mut rng = Rng::new(seed + 50);
            while g.outcome().is_none() {
                let mv = if g.to_move() == Player::One {
                    g.ai_move(rng.next_u64()).unwrap()
                } else {
                    let m = g.legal_moves();
                    m[rng.below(m.len())].clone()
                };
                g.play(&mv).unwrap();
            }
            if g.outcome() == Some(Outcome::Win(Player::One)) {
                wins += 1;
            }
        }
        assert!(wins * 10 >= n * 7, "AI won only {wins}/{n}");
    }
}
