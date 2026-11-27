//! Fitil — joc de cărți cu o bombă în pachet, variantă originală pentru doi.
//!
//! Pachetul ascunde o singură Bombă. Fiecare tură poți juca orice cărți, apoi trebuie
//! să tragi o carte (încheie tura). Dacă tragi Bomba și nu ai Dezamorsare, pierzi; cu o
//! Dezamorsare o pui înapoi în pachet unde vrei (adâncimea 0 = deasupra).
//!
//! Cărți: Dezamorsare, Atac (încheie tura fără să tragi; adversarul trage de două ori),
//! Sari (încheie o tură fără tragere), Amestecă (amestecă pachetul), Privire (vezi primele 3
//! cărți), Cere (adversarul îți dă o carte la alegerea lui) și trei feluri de Mâți: două
//! cărți identice = furi o carte la întâmplare din mâna adversarului.
//! Simplificări: fără „Nu”; un Atac peste un Atac adaugă 2 trageri celor rămase.
//!
//! Informație ascunsă: vezi doar mâna Jucătorului 1, Privirea ta și poziția Bombei pusă de tine.
//! AI-ul nu citește mâna adversarului, nici pachetul: estimează riscul din ce știe.
#![allow(clippy::needless_range_loop, clippy::doc_lazy_continuation)]

use crate::cards::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum K {
    Bomba,
    Dezamorsare,
    Atac,
    Sari,
    Amesteca,
    Privire,
    Cere,
    MatA,
    MatB,
    MatC,
}

impl K {
    pub fn name(self) -> &'static str {
        match self {
            K::Bomba => "bomba",
            K::Dezamorsare => "dezamorsare",
            K::Atac => "atac",
            K::Sari => "sari",
            K::Amesteca => "amesteca",
            K::Privire => "privire",
            K::Cere => "cere",
            K::MatA => "A",
            K::MatB => "B",
            K::MatC => "C",
        }
    }
    fn value(self) -> i32 {
        match self {
            K::Dezamorsare => 200,
            K::Sari => 40,
            K::Atac => 38,
            K::Privire => 15,
            K::Amesteca => 12,
            K::Cere => 18,
            _ => 8,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Play(K),
    Pair(K),
    Draw,
    Insert(u8),
    Give(K),
    /// Bomba trasă fără Dezamorsare explodează (mutare forțată: căutarea nu vede cartea din vârful pachetului).
    Explode,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Turn,
    Insert,
    Give {
        to: usize,
    },
    /// Ai tras Bomba fără Dezamorsare: urmează `Move::Explode`.
    Boom,
}

#[derive(Clone)]
pub struct Fitil {
    /// Whose private info `render` shows (`None` = the mover's, hot-seat).
    viewer: Option<Player>,
    deck: Vec<K>, // vârful = ultimul element
    discard: Vec<K>,
    hands: [Vec<K>; 2],
    side: Player,
    /// Câte trageri mai are de făcut jucătorul la mutare.
    need: u8,
    phase: Phase,
    winner: Option<Player>,
    /// Adâncimea Bombei (0 = vârf) cunoscută de fiecare jucător.
    bomb_known: [Option<usize>; 2],
    /// Primele cărți cunoscute de fiecare jucător (vârful primul).
    peek: [Vec<K>; 2],
    rng: Rng,
    heur: (usize, i32),
}

impl Default for Fitil {
    fn default() -> Self {
        Self::new()
    }
}

fn risk_cost(has_defuse: bool) -> i32 {
    if has_defuse {
        30
    } else {
        1000
    }
}

impl Fitil {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut pool = Vec::new();
        for (k, n) in [
            (K::Atac, 4),
            (K::Sari, 4),
            (K::Amesteca, 4),
            (K::Privire, 5),
            (K::Cere, 4),
            (K::MatA, 4),
            (K::MatB, 4),
            (K::MatC, 4),
        ] {
            for _ in 0..n {
                pool.push(k);
            }
        }
        shuffle(&mut pool, &mut rng);
        let mut hands = [vec![K::Dezamorsare], vec![K::Dezamorsare]];
        for _ in 0..4 {
            hands[0].push(pool.pop().unwrap());
            hands[1].push(pool.pop().unwrap());
        }
        pool.push(K::Dezamorsare);
        pool.push(K::Dezamorsare);
        pool.push(K::Bomba);
        shuffle(&mut pool, &mut rng);
        for h in &mut hands {
            h.sort_by_key(|k| *k as u8);
        }
        Fitil {
            viewer: None,
            deck: pool,
            discard: Vec::new(),
            hands,
            side: Player::One,
            need: 1,
            phase: Phase::Turn,
            winner: None,
            bomb_known: [None; 2],
            peek: [Vec::new(), Vec::new()],
            rng,
            heur: (0, 0),
        }
    }

    pub fn total_cards(&self) -> usize {
        self.deck.len()
            + self.discard.len()
            + self.hands[0].len()
            + self.hands[1].len()
            + usize::from(self.phase == Phase::Insert)
    }

    fn count(&self, p: usize, k: K) -> usize {
        self.hands[p].iter().filter(|x| **x == k).count()
    }

    /// Riscul ca vârful pachetului să fie Bomba, din punctul de vedere al lui `p`.
    fn risk(&self, p: usize) -> f64 {
        let n = self.deck.len();
        if n == 0 {
            return 0.0;
        }
        if let Some(d) = self.bomb_known[p] {
            return if d == 0 { 1.0 } else { 0.0 };
        }
        if let Some(k) = self.peek[p].first() {
            return if *k == K::Bomba { 1.0 } else { 0.0 };
        }
        // Cărțile văzute în top nu sunt bombă: restul au probabilitate egală.
        let known = self.peek[p].len().min(n);
        1.0 / (n - known).max(1) as f64
    }

    fn move_score(&self, p: usize, m: &Move) -> i32 {
        let risk = self.risk(p);
        let c = risk_cost(self.count(p, K::Dezamorsare) > 0);
        let opp = 1 - p;
        let opp_cards = self.hands[opp].len() as i32;
        let own_after = |k: K| (self.count(p, k) as i32 - 1).max(0);
        match *m {
            Move::Draw => (-(risk * f64::from(c))) as i32,
            Move::Play(K::Sari) => (risk * f64::from(c)) as i32 - 3 - own_after(K::Sari),
            Move::Play(K::Atac) => (risk * f64::from(c)) as i32 + 7 - own_after(K::Atac),
            Move::Play(K::Privire) => {
                if self.peek[p].is_empty() && self.bomb_known[p].is_none() {
                    12
                } else {
                    -6
                }
            }
            Move::Play(K::Amesteca) => {
                // Util când știu că Bomba e în vârf.
                (risk * f64::from(c)) as i32 - 2
            }
            Move::Play(K::Cere) => {
                if opp_cards > 0 {
                    9
                } else {
                    -9
                }
            }
            Move::Pair(_) => {
                if opp_cards > 0 {
                    11
                } else {
                    -9
                }
            }
            Move::Insert(d) => {
                let deeper = self.need > 1;
                if deeper {
                    i32::from(d)
                } else {
                    -i32::from(d)
                }
            }
            Move::Give(k) => -k.value(),
            Move::Explode => 0,
            Move::Play(_) => -5,
        }
    }

    fn remove(&mut self, p: usize, k: K) {
        let i = self.hands[p]
            .iter()
            .position(|x| *x == k)
            .expect("card in hand");
        self.hands[p].remove(i);
    }

    fn advance_draw(&mut self) {
        // O tragere (sau tură sărită) s-a încheiat.
        if self.need > 0 {
            self.need -= 1;
        }
        if self.need == 0 {
            self.side = self.side.other();
            self.need = 1;
        }
    }

    fn after_top_removed(&mut self) {
        for p in 0..2 {
            if !self.peek[p].is_empty() {
                self.peek[p].remove(0);
            }
            self.bomb_known[p] = match self.bomb_known[p] {
                Some(0) | None => None,
                Some(d) => Some(d - 1),
            };
        }
    }
}

impl Game for Fitil {
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.winner.is_some() {
            return Vec::new();
        }
        let p = self.side.idx();
        match self.phase {
            Phase::Boom => vec![Move::Explode],
            Phase::Insert => (0..=self.deck.len())
                .map(|d| Move::Insert(d as u8))
                .collect(),
            Phase::Give { .. } => {
                let mut v: Vec<K> = Vec::new();
                for &k in &self.hands[p] {
                    if !v.contains(&k) {
                        v.push(k);
                    }
                }
                v.into_iter().map(Move::Give).collect()
            }
            Phase::Turn => {
                let mut v = vec![Move::Draw];
                for k in [K::Atac, K::Sari, K::Amesteca, K::Privire, K::Cere] {
                    if self.count(p, k) > 0 {
                        v.push(Move::Play(k));
                    }
                }
                for k in [K::MatA, K::MatB, K::MatC] {
                    if self.count(p, k) >= 2 && !self.hands[1 - p].is_empty() {
                        v.push(Move::Pair(k));
                    }
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        let p = self.side.idx();
        let o = 1 - p;
        self.heur = (p, self.move_score(p, m));
        match *m {
            Move::Draw => {
                let c = self.deck.pop().expect("deck");
                self.after_top_removed();
                if c == K::Bomba {
                    if self.count(p, K::Dezamorsare) > 0 {
                        self.remove(p, K::Dezamorsare);
                        self.discard.push(K::Dezamorsare);
                        self.phase = Phase::Insert;
                    } else {
                        self.discard.push(K::Bomba);
                        self.phase = Phase::Boom;
                    }
                } else {
                    self.hands[p].push(c);
                    self.hands[p].sort_by_key(|k| *k as u8);
                    self.advance_draw();
                }
            }
            Move::Explode => {
                self.winner = Some(self.side.other());
            }
            Move::Insert(d) => {
                let d = d as usize;
                let at = self.deck.len() - d;
                self.deck.insert(at, K::Bomba);
                self.peek = [Vec::new(), Vec::new()];
                self.bomb_known = [None, None];
                self.bomb_known[p] = Some(d);
                self.phase = Phase::Turn;
                self.advance_draw();
            }
            Move::Give(k) => {
                if let Phase::Give { to } = self.phase {
                    self.remove(p, k);
                    self.hands[to].push(k);
                    self.hands[to].sort_by_key(|x| *x as u8);
                    self.side = if to == 0 { Player::One } else { Player::Two };
                    self.phase = Phase::Turn;
                }
            }
            Move::Pair(k) => {
                self.remove(p, k);
                self.remove(p, k);
                self.discard.push(k);
                self.discard.push(k);
                let i = self.rng.below(self.hands[o].len());
                let c = self.hands[o].remove(i);
                self.hands[p].push(c);
                self.hands[p].sort_by_key(|x| *x as u8);
            }
            Move::Play(k) => {
                self.remove(p, k);
                self.discard.push(k);
                match k {
                    K::Sari => self.advance_draw(),
                    K::Atac => {
                        let extra = self.need.saturating_sub(1);
                        self.side = self.side.other();
                        self.need = 2 + extra;
                    }
                    K::Amesteca => {
                        let mut d = std::mem::take(&mut self.deck);
                        shuffle(&mut d, &mut self.rng);
                        self.deck = d;
                        self.peek = [Vec::new(), Vec::new()];
                        self.bomb_known = [None, None];
                    }
                    K::Privire => {
                        let n = self.deck.len().min(3);
                        let top: Vec<K> =
                            (0..n).map(|i| self.deck[self.deck.len() - 1 - i]).collect();
                        if let Some(d) = top.iter().position(|x| *x == K::Bomba) {
                            self.bomb_known[p] = Some(d);
                        }
                        self.peek[p] = top;
                    }
                    K::Cere if !self.hands[o].is_empty() => {
                        self.phase = Phase::Give { to: p };
                        self.side = self.side.other();
                    }
                    _ => {}
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.winner.map(Outcome::Win)
    }

    fn evaluate(&self, p: Player) -> i32 {
        if self.heur.0 == p.idx() {
            self.heur.1
        } else {
            -self.heur.1
        }
    }

    /// Adâncime 0 (o singură mutare): `AlphaBeta(1)` ar fi extins la Hard/Expert răspunsul adversarului
    /// din mâna lui reală; evaluarea (`heur`) citește doar ce știe jucătorul care mută.
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }

    fn render(&self) -> String {
        let mut s = String::new();
        let list = |v: &[K]| {
            if v.is_empty() {
                "—".to_string()
            } else {
                v.iter().map(|k| k.name()).collect::<Vec<_>>().join(" ")
            }
        };
        s.push_str(&format!(
            "Fitil — pachet: {} cărți | aruncate: {}\n",
            self.deck.len(),
            self.discard.len()
        ));
        if let Some(l) = self.discard.last() {
            s.push_str(&format!("Ultima aruncată: {}\n", l.name()));
        }
        let v = self.viewer.unwrap_or_else(|| self.to_move()).idx();
        s.push_str(&format!("Mâna ta (J{}): {}\n", v + 1, list(&self.hands[v])));
        s.push_str(&format!(
            "Adversarul are {} cărți.\n",
            self.hands[1 - v].len()
        ));
        if !self.peek[0].is_empty() {
            s.push_str(&format!(
                "Știi vârful pachetului: {}\n",
                list(&self.peek[0])
            ));
        }
        if let Some(d) = self.bomb_known[0] {
            s.push_str(&format!("Știi unde e Bomba: adâncimea {d} (0 = vârf).\n"));
        }
        if self.winner.is_some() {
            s.push_str("Joc încheiat: a explodat o bombă.\n");
            return s;
        }
        match self.phase {
            Phase::Insert => s.push_str(&format!(
                "Mută J{}: ai tras Bomba și ai dezamorsat-o. Repune-o: dezamorsez 0..{} (0 = vârf).\n",
                self.side.idx() + 1,
                self.deck.len()
            )),
            Phase::Boom => s.push_str(&format!(
                "Mută J{}: ai tras Bomba și nu ai Dezamorsare! Scrie «explodeaza».\n",
                self.side.idx() + 1
            )),
            Phase::Give { to } => s.push_str(&format!(
                "Mută J{}: dă o carte lui J{} (dau <carte>).\n",
                self.side.idx() + 1,
                to + 1
            )),
            Phase::Turn => s.push_str(&format!(
                "Mută J{} ({} trageri de făcut). Mutări: juc atac|sari|amesteca|privire|cere, pereche A|B|C, trag.\n",
                self.side.idx() + 1,
                self.need
            )),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Draw => "trag".into(),
            Move::Explode => "explodeaza".into(),
            Move::Insert(d) => format!("dezamorsez {d}"),
            Move::Give(k) => format!("dau {}", k.name()),
            Move::Pair(k) => format!("pereche {}", k.name()),
            Move::Play(k) => format!("juc {}", k.name()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_and_conservation() {
        let g = Fitil::with_seed(1);
        assert_eq!(g.total_cards(), 33 + 2 + 2 + 1);
        assert_eq!(g.hands[0].len(), 5);
        assert!(g.deck.contains(&K::Bomba));
        for seed in 1..50 {
            let mut g = Fitil::with_seed(seed);
            let total = g.total_cards();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                assert_eq!(g.total_cards(), total);
                n += 1;
                assert!(n < 3000);
            }
        }
    }

    #[test]
    fn bomb_without_defuse_loses() {
        let mut g = Fitil::with_seed(2);
        g.hands[0].retain(|k| *k != K::Dezamorsare);
        g.deck.push(K::Bomba);
        g.apply(&Move::Draw);
        assert!(g.outcome().is_none());
        assert_eq!(g.legal_moves(), vec![Move::Explode]);
        g.apply(&Move::Explode);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn defuse_and_reinsert() {
        let mut g = Fitil::with_seed(2);
        g.deck.retain(|k| *k != K::Bomba);
        g.deck.push(K::Bomba);
        g.apply(&Move::Draw);
        assert!(g.outcome().is_none());
        let moves = g.legal_moves();
        assert_eq!(moves.len(), g.deck.len() + 1);
        g.apply(&Move::Insert(3));
        assert_eq!(
            g.deck.iter().position(|k| *k == K::Bomba),
            Some(g.deck.len() - 1 - 3)
        );
        assert_eq!(g.to_move(), Player::Two);
        assert_eq!(g.bomb_known[0], Some(3));
        assert_eq!(g.bomb_known[1], None);
    }

    #[test]
    fn attack_gives_two_draws() {
        let mut g = Fitil::with_seed(3);
        g.hands[0].push(K::Atac);
        g.apply(&Move::Play(K::Atac));
        assert_eq!(g.to_move(), Player::Two);
        assert_eq!(g.need, 2);
        g.deck.retain(|k| *k != K::Bomba);
        g.deck.insert(0, K::Bomba);
        g.apply(&Move::Draw);
        assert_eq!(g.to_move(), Player::Two);
        g.apply(&Move::Draw);
        assert_eq!(g.to_move(), Player::One);
    }

    #[test]
    fn favor_and_skip() {
        let mut g = Fitil::with_seed(4);
        g.hands[0].push(K::Cere);
        g.hands[0].push(K::Sari);
        let n0 = g.hands[0].len();
        g.apply(&Move::Play(K::Cere));
        assert_eq!(g.to_move(), Player::Two);
        let l = g.legal_moves();
        g.apply(&l[0]);
        assert_eq!(g.to_move(), Player::One);
        assert_eq!(g.hands[0].len(), n0); // -1 Cere, +1 primită
        g.apply(&Move::Play(K::Sari));
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn ai_beats_random() {
        let mut wins = 0;
        for seed in 1..=60u64 {
            let mut g = Fitil::with_seed(seed);
            let mut rng = Rng::new(seed + 5);
            while g.outcome().is_none() {
                let m = if g.to_move() == Player::One {
                    polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
                } else {
                    let l = g.legal_moves();
                    l[rng.below(l.len())]
                };
                g.apply(&m);
            }
            if g.outcome() == Some(Outcome::Win(Player::One)) {
                wins += 1;
            }
        }
        assert!(wins >= 36, "wins {wins}");
    }
}

#[cfg(test)]
mod a2_audit {
    use super::*;

    #[test]
    fn ai_nu_vede_mana_adversarului_nici_pachetul_la_orice_nivel() {
        crate::p3_comun::ai_blind_check(
            "fitil",
            Fitil::with_seed,
            |g, me, seed| {
                // ce nu vede jucătorul: mâna adversarului și pachetul; rămân la loc cărțile pe care le-a privit
                // și adâncimea Bombei pe care o știe
                let (m, o) = (me.idx(), 1 - me.idx());
                let len = g.deck.len();
                let known_peek = g.peek[m].len().min(len);
                let known_bomb = g.bomb_known[m];
                let is_known = |d: usize| d < known_peek || known_bomb == Some(d);
                let n = g.hands[o].len();
                let mut rest: Vec<K> = g.hands[o].clone();
                for d in (0..len).filter(|&d| !is_known(d)) {
                    rest.push(g.deck[len - 1 - d]);
                }
                crate::p3_comun::shuffle_with(&mut rest, seed);
                g.hands[o] = rest.drain(..n).collect();
                g.hands[o].sort_by_key(|k| *k as u8);
                for d in (0..len).filter(|&d| !is_known(d)) {
                    g.deck[len - 1 - d] = rest.pop().unwrap();
                }
                g.rng = Rng::new(seed ^ 0xF171);
            },
            12,
        );
    }
}
