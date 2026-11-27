//! „Trezirea Reginelor” — variantă originală C4 (2 jucători, mâini ascunse, `set_viewer`).
//! 12 regine dorm cu fața în jos (`rei s` o trezește la mutarea forțată `arata`, deci valoarea nu se vede dinainte).
//! Cărți: regi (trezesc), cavaleri (fură o regină trează; un dragon al adversarului o apără automat),
//! poțiuni (adorm o regină a adversarului; o baghetă o apără automat), numere (aruncă 1 carte, o pereche egală
//! sau o sumă `a b c` cu a+b=c). Câștigi cu 50 de puncte sau 5 regine; la plafon de mutări, cine are mai multe puncte.
//! AI-ul nu citește mâna adversarului și nici ordinea pachetului: evaluarea folosește doar reginele treze.
use crate::c4_util::*;

const KING: u8 = 0;
const KNIGHT: u8 = 1;
const DRAGON: u8 = 2;
const POTION: u8 = 3;
const WAND: u8 = 4;
const QV: [i32; 12] = [5, 5, 5, 5, 10, 10, 10, 10, 15, 15, 20, 20];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Pend {
    Wake(usize),
    Steal(usize),
    Sleep(usize),
}

#[derive(Clone)]
pub struct Regine {
    dice: Dice,
    deck: Vec<u8>,
    discard: Vec<u8>,
    hand: [Vec<u8>; 2],
    asleep: Vec<i32>,
    awake: [Vec<i32>; 2],
    turn: usize,
    pend: Option<Pend>,
    viewer: Option<Player>,
    plies: u32,
}

fn cname(c: u8) -> String {
    match c {
        KING => "rege".into(),
        KNIGHT => "cavaler".into(),
        DRAGON => "dragon".into(),
        POTION => "poțiune".into(),
        WAND => "baghetă".into(),
        n => format!("{}", n - 4),
    }
}

impl Regine {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = polytabletop_core::rng::Rng::new(seed);
        let mut deck: Vec<u8> = Vec::new();
        for (c, n) in [(KING, 6), (KNIGHT, 4), (DRAGON, 3), (POTION, 3), (WAND, 3)] {
            deck.extend(std::iter::repeat_n(c, n));
        }
        for v in 5..=12u8 {
            deck.extend([v; 3]);
        }
        shuf(&mut deck, &mut rng);
        let mut q = QV.to_vec();
        shuf(&mut q, &mut rng);
        let h0: Vec<u8> = deck.split_off(deck.len() - 5);
        let h1: Vec<u8> = deck.split_off(deck.len() - 5);
        Regine {
            dice: Dice::new(seed ^ 0x77),
            deck,
            discard: Vec::new(),
            hand: [h0, h1],
            asleep: q,
            awake: [Vec::new(), Vec::new()],
            turn: 0,
            pend: None,
            viewer: None,
            plies: 0,
        }
    }
    fn points(&self, w: usize) -> i32 {
        self.awake[w].iter().sum()
    }
    fn draw_to_five(&mut self, w: usize) {
        while self.hand[w].len() < 5 {
            if self.deck.is_empty() {
                if self.discard.is_empty() {
                    return;
                }
                self.deck = std::mem::take(&mut self.discard);
                for i in (1..self.deck.len()).rev() {
                    let j = self.dice.below(i + 1);
                    self.deck.swap(i, j);
                }
            }
            let c = self.deck.pop().unwrap();
            self.hand[w].push(c);
        }
    }
    fn take(&mut self, w: usize, c: u8) {
        let p = self.hand[w].iter().position(|&x| x == c).unwrap();
        self.hand[w].remove(p);
        self.discard.push(c);
    }
    fn end_turn(&mut self) {
        let me = self.turn;
        self.draw_to_five(me);
        self.turn = 1 - me;
        self.plies += 1;
    }
}

impl Default for Regine {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Regine {
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
        let h = &self.hand[self.turn];
        let mut v: Vec<String> = Vec::new();
        let mut d = h.clone();
        d.sort();
        d.dedup();
        for &c in &d {
            match c {
                KING => {
                    for s in 0..self.asleep.len() {
                        v.push(format!("rei {}", s + 1));
                    }
                }
                KNIGHT => {
                    for q in 0..self.awake[1 - self.turn].len() {
                        v.push(format!("cav {}", q + 1));
                    }
                }
                POTION => {
                    for q in 0..self.awake[1 - self.turn].len() {
                        v.push(format!("pot {}", q + 1));
                    }
                }
                _ => {}
            }
            v.push(format!("arunc {}", cname(c)));
            if c >= 5 && h.iter().filter(|&&x| x == c).count() >= 2 {
                v.push(format!("arunc {} {}", cname(c), cname(c)));
            }
        }
        for &a in &d {
            for &b in &d {
                if a >= 5 && b >= 5 && a <= b {
                    let need = if a == b { 2 } else { 1 };
                    if h.iter().filter(|&&x| x == a).count() < need {
                        continue;
                    }
                    let s = (a - 4) + (b - 4);
                    if s <= 8 {
                        let c = s + 4;
                        let mut hh = h.clone();
                        for x in [a, b] {
                            if let Some(p) = hh.iter().position(|&y| y == x) {
                                hh.remove(p);
                            }
                        }
                        if hh.contains(&c) {
                            v.push(format!("suma {} {} {}", cname(a), cname(b), cname(c)));
                        }
                    }
                }
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = self.turn;
        if m == "arata" {
            match self.pend.take().unwrap() {
                Pend::Wake(s) => {
                    let q = self.asleep.remove(s);
                    self.awake[me].push(q);
                }
                Pend::Steal(q) => {
                    if self.hand[1 - me].contains(&DRAGON) {
                        self.take(1 - me, DRAGON);
                    } else {
                        let v = self.awake[1 - me].remove(q);
                        self.awake[me].push(v);
                    }
                }
                Pend::Sleep(q) => {
                    if self.hand[1 - me].contains(&WAND) {
                        self.take(1 - me, WAND);
                    } else {
                        let v = self.awake[1 - me].remove(q);
                        self.asleep.push(v);
                    }
                }
            }
            self.end_turn();
            return;
        }
        let p: Vec<&str> = m.split(' ').collect();
        let num = |s: &str| s.parse::<usize>().unwrap() - 1;
        let card = |s: &str| -> u8 {
            match s {
                "rege" => KING,
                "cavaler" => KNIGHT,
                "dragon" => DRAGON,
                "poțiune" => POTION,
                "baghetă" => WAND,
                n => n.parse::<u8>().unwrap() + 4,
            }
        };
        match p[0] {
            "rei" => {
                self.take(me, KING);
                self.pend = Some(Pend::Wake(num(p[1])));
            }
            "cav" => {
                self.take(me, KNIGHT);
                self.pend = Some(Pend::Steal(num(p[1])));
            }
            "pot" => {
                self.take(me, POTION);
                self.pend = Some(Pend::Sleep(num(p[1])));
            }
            "arunc" => {
                for s in &p[1..] {
                    let c = card(s);
                    self.take(me, c);
                }
                self.end_turn();
            }
            _ => {
                for s in &p[1..] {
                    let c = card(s);
                    self.take(me, c);
                }
                self.end_turn();
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.pend.is_some() {
            return None;
        }
        for w in 0..2 {
            if self.points(w) >= 50 || self.awake[w].len() >= 5 {
                return Some(Outcome::Win(pl(w)));
            }
        }
        if self.asleep.is_empty() || self.plies >= 240 {
            return Some(pts_outcome([self.points(0), self.points(1)]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let mut v = 100 * (self.points(a) - self.points(1 - a));
        if let Some(pd) = self.pend {
            let sign = if self.turn == a { 1 } else { -1 };
            v += sign
                * match pd {
                    Pend::Wake(_) => 1200,
                    Pend::Steal(q) => 7 * self.awake[1 - self.turn][q] * 100 / 10,
                    Pend::Sleep(q) => 6 * self.awake[1 - self.turn][q] * 100 / 10,
                };
        }
        v
    }
    fn show(&self) -> String {
        let viewer = self.viewer.map(pi).unwrap_or(self.turn);
        let mut s = format!("Trezirea Reginelor — mutarea {}/240\n", self.plies + 1);
        s.push_str(&format!(
            "Regine adormite: {} (cu fața în jos)\n",
            self.asleep.len()
        ));
        for w in 0..2 {
            let q: Vec<String> = self.awake[w].iter().map(|x| x.to_string()).collect();
            s.push_str(&format!(
                "J{}: {} puncte, regine [{}], {} cărți\n",
                w + 1,
                self.points(w),
                q.join(" "),
                self.hand[w].len()
            ));
        }
        let mut h = self.hand[viewer].clone();
        h.sort();
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            viewer + 1,
            h.iter().map(|&c| cname(c)).collect::<Vec<_>>().join(", ")
        ));
        s.push_str(&format!(
            "La mutare: {} (rei s | cav q | pot q | arunc c [c] | suma a b c | arata)\n",
            pname(self.side())
        ));
        s
    }
    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
}

pub fn make() -> Box<dyn polytabletop_core::GameObj> {
    Box::new(B(Regine::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plays() {
        check(Regine::with_seed, 900);
    }

    #[test]
    fn hidden_information_and_ai_honesty() {
        let g = Regine::with_seed(3);
        let mut h = g.clone();
        // schimbă mâna adversarului și pachetul: AI-ul trebuie să aleagă la fel
        h.hand[1] = vec![DRAGON, WAND, 5, 6, 7];
        h.deck.reverse();
        for seed in 1..6 {
            assert_eq!(ai_of(&g, seed), ai_of(&h, seed));
        }
        let mut v = g.clone();
        v.view(Some(Player::One));
        let s = v.show();
        assert!(s.contains("Mâna ta (J1)") && !s.contains("Mâna ta (J2)"));
    }
}
