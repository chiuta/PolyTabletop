//! Jaful din tren (variantă proprie de programare de acțiuni, 2 jucători, informație ascunsă):
//! trenul are 4 vagoane (1–4), fiecare cu interior și acoperiș, cu pungi de bani în ele. Cei
//! doi bandiți pornesc înăuntru (J1 în vagonul 4, J2 în vagonul 3). Patru runde. În fiecare rundă
//! fiecare jucător își alege în secret un program de 3 acțiuni (J1 întâi, J2 apoi; J2 nu vede
//! programul lui J1): `s` mută un vagon spre stânga (1), `d` spre dreapta (4), `u` urcă pe
//! acoperiș sau coboară în interior, `t` trage în celălalt bandit dacă e pe același nivel și în
//! același vagon (pe acoperiș: în orice vagon de pe acoperiș), `j` ia cea mai mare pungă de pe
//! nivelul și din vagonul tău. Apoi acțiunile se joacă pe rând: prima a lui J1, prima a lui J2,
//! a doua a lui J1 etc. Cine e împușcat scapă pe loc cea mai mare pungă a lui (rămâne în vagonul
//! și pe nivelul unde se află); fiecare bandit are 4 gloanțe. La final câștigă cine are mai mulți
//! bani. Mutări: programul de 3 litere, ex. `ddj`, `utj`.
#![allow(clippy::all)]

use crate::cards::{handover_banner, Viewer};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const ACT: [char; 5] = ['s', 'd', 'u', 't', 'j'];

#[derive(Clone)]
pub struct TrenJaf {
    loot: [[Vec<i32>; 2]; 4], // [vagon][0 interior, 1 acoperiș]
    pos: [(i8, u8); 2],       // vagon, nivel (0 interior, 1 acoperiș)
    purse: [Vec<i32>; 2],
    ammo: [u8; 2],
    round: u8,
    prog: [Option<[u8; 3]>; 2],
    side: Player,
    prev: Player,
    last: String,
    viewer: Viewer,
    fresh: Option<[i32; 2]>,
}

impl Default for TrenJaf {
    fn default() -> Self {
        Self::new()
    }
}

impl TrenJaf {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let vals = [250, 300, 400, 500, 650];
        let mut loot: [[Vec<i32>; 2]; 4] = Default::default();
        for car in loot.iter_mut() {
            for _ in 0..2 {
                car[0].push(vals[rng.below(vals.len())]);
            }
        }
        loot[1][1].push(600);
        loot[2][1].push(600);
        TrenJaf {
            loot,
            pos: [(3, 0), (2, 0)],
            purse: [vec![], vec![]],
            ammo: [4, 4],
            round: 0,
            prog: [None, None],
            side: Player::One,
            prev: Player::One,
            last: String::new(),
            viewer: Viewer::Fixed(Player::One),
            fresh: None,
        }
    }
    fn total(&self, p: usize) -> i32 {
        self.purse[p].iter().sum()
    }
    fn step(&mut self, p: usize, a: u8) {
        let q = 1 - p;
        match a {
            0 => self.pos[p].0 = (self.pos[p].0 - 1).max(0),
            1 => self.pos[p].0 = (self.pos[p].0 + 1).min(3),
            2 => self.pos[p].1 = 1 - self.pos[p].1,
            3 => {
                if self.ammo[p] > 0 {
                    self.ammo[p] -= 1;
                    let same_level = self.pos[p].1 == self.pos[q].1;
                    let near = self.pos[p].0 == self.pos[q].0 || self.pos[p].1 == 1;
                    if same_level && near && !self.purse[q].is_empty() {
                        let k = (0..self.purse[q].len())
                            .max_by_key(|&i| self.purse[q][i])
                            .unwrap();
                        let v = self.purse[q].remove(k);
                        self.loot[self.pos[q].0 as usize][self.pos[q].1 as usize].push(v);
                    }
                }
            }
            4 => {
                let l = &mut self.loot[self.pos[p].0 as usize][self.pos[p].1 as usize];
                if let Some(k) = (0..l.len()).max_by_key(|&i| l[i]) {
                    let v = l.remove(k);
                    self.purse[p].push(v);
                }
            }
            _ => {}
        }
    }
    fn resolve(&mut self, a: [u8; 3], b: [u8; 3]) {
        for k in 0..3 {
            self.step(0, a[k]);
            self.step(1, b[k]);
        }
    }
    fn decode(m: u8) -> [u8; 3] {
        [m / 25, (m / 5) % 5, m % 5]
    }
}

impl Game for TrenJaf {
    type Move = u8;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<u8> {
        if self.round >= 4 {
            return vec![];
        }
        (0..125).collect()
    }
    fn apply(&mut self, m: &u8) {
        let p = self.side.idx();
        self.fresh = None;
        self.prog[p] = Some(Self::decode(*m));
        self.prev = self.side;
        if p == 0 {
            self.side = Player::Two;
            return;
        }
        // J2 a ales: evaluarea cinstită simulează programul lui J2 contra unui J1 care nu face nimic
        let mut probe = self.clone();
        probe.resolve([9, 9, 9], Self::decode(*m));
        self.fresh = Some([probe.total(0), probe.total(1)]);
        let (a, b) = (self.prog[0].unwrap(), self.prog[1].unwrap());
        let before = (self.total(0), self.total(1));
        self.resolve(a, b);
        let name = |p: [u8; 3]| p.iter().map(|&x| ACT[x as usize]).collect::<String>();
        self.last = format!(
            "Runda {}: J1 {} → {} (+{}), J2 {} → {} (+{})",
            self.round + 1,
            name(a),
            self.total(0),
            self.total(0) - before.0,
            name(b),
            self.total(1),
            self.total(1) - before.1
        );
        self.prog = [None, None];
        self.round += 1;
        self.side = Player::One;
    }
    fn outcome(&self) -> Option<Outcome> {
        if self.round < 4 {
            return None;
        }
        Some(match self.total(0).cmp(&self.total(1)) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            _ => Outcome::Draw,
        })
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let v = self.fresh.unwrap_or([self.total(0), self.total(1)]);
        let mut s = v[me] - v[1 - me];
        // apropiere de pungi (informație publică)
        let (c, l) = (self.pos[me].0 as usize, self.pos[me].1 as usize);
        s += self.loot[c][l].iter().max().copied().unwrap_or(0) / 4;
        s
    }
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }
    fn render(&self) -> String {
        let mut s = String::new();
        if self.prev != self.side && self.outcome().is_none() {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        s.push_str(&format!("Runda {}/4\n", (self.round + 1).min(4)));
        for c in 0..4usize {
            let mut here = String::new();
            for p in 0..2 {
                if self.pos[p].0 as usize == c {
                    here.push_str(&format!(
                        " J{}{}",
                        p + 1,
                        if self.pos[p].1 == 1 { "^" } else { "" }
                    ));
                }
            }
            s.push_str(&format!(
                "Vagon {}: interior {:?}, acoperiș {:?}{}\n",
                c + 1,
                self.loot[c][0],
                self.loot[c][1],
                here
            ));
        }
        for p in 0..2 {
            s.push_str(&format!(
                "J{}: {} bani ({} pungi), {} gloanțe\n",
                p + 1,
                self.total(p),
                self.purse[p].len(),
                self.ammo[p]
            ));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        let v = self.viewer.who(self.side);
        let line = match self.prog[v.idx()] {
            Some(a) => a.iter().map(|&x| ACT[x as usize]).collect::<String>(),
            None => "(nealeasă)".into(),
        };
        s.push_str(&format!("Programul tău (J{}): {}\n", v.idx() + 1, line));
        s.push_str(&format!(
            "La mutare: Jucătorul {} (alege 3 acțiuni din s d u t j)\n",
            self.side.idx() + 1
        ));
        s
    }
    fn format_move(&self, m: &u8) -> String {
        Self::decode(*m).iter().map(|&x| ACT[x as usize]).collect()
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    #[test]
    fn actions_resolve_in_turn() {
        let mut g = TrenJaf::with_seed(1);
        g.loot = Default::default();
        g.loot[3][0] = vec![500];
        g.apply(&(4 * 25 + 4 * 5 + 4)); // J1: jjj
        assert_eq!(g.to_move(), Player::Two);
        g.apply(&(3 * 25 + 3 * 5 + 3)); // J2: ttt, în vagonul 3 interior, J1 e în vagonul 4: nu nimerește
        assert_eq!(g.total(0), 500);
        assert_eq!(g.round, 1);
        assert_eq!(g.to_move(), Player::One);
    }
    #[test]
    fn shot_drops_purse() {
        let mut g = TrenJaf::with_seed(2);
        g.loot = Default::default();
        g.pos = [(2, 0), (2, 0)];
        g.purse[0] = vec![300, 500];
        g.step(1, 3);
        assert_eq!(g.purse[0], vec![300]);
        assert_eq!(g.loot[2][0], vec![500]);
    }
    #[test]
    fn hidden_program() {
        let mut g = TrenJaf::with_seed(3);
        g.apply(&7);
        let f = |who| {
            let mut x = g.clone();
            x.set_viewer(Some(who));
            x.render()
                .lines()
                .find(|l| l.starts_with("Programul tău"))
                .unwrap()
                .to_string()
        };
        assert!(!f(Player::Two).contains("(nealeasă)") == false);
        assert!(!f(Player::One).contains("(nealeasă)"));
    }
    #[test]
    fn random_and_ai() {
        check(TrenJaf::with_seed, 20);
    }
}
