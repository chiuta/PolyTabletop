//! Ziua de Salariu — cursă de buget pe trei luni (variantă originală, 2 jucători).
//!
//! Tabla are 3 luni × 30 de zile. Zarul (`zar`, mutare forțată) te mută; ziua unde aterizezi:
//! poștă (o scrisoare: factură sau cadou, −6..+4 monede), ofertă (poți `cumpar` un lot cu 5 monede
//! ca să-l vinzi mai târziu cu 9, sau `las`), vânzare (vinzi automat toate loturile cu 9 monede
//! fiecare), ziua 30 a lunii = salariu (+8 monede). Dacă traversezi o zi de salariu o primești oricum.
//! Banii nu pot coborî sub 0 (ce nu ai nu mai plătești). Câștigă, după trei luni, cine are mai mulți
//! bani (loturile nevândute valorează 3 fiecare).
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const DAYS: usize = 90;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Sp {
    Plain,
    Mail,
    Deal,
    Sell,
}

#[derive(Clone)]
pub struct Salariu {
    board: Vec<Sp>,
    pos: [usize; 2],
    cash: [i32; 2],
    lots: [i32; 2],
    offer: bool,
    side: Player,
    rng: Rng,
}

impl Salariu {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut board = Vec::new();
        for _ in 0..DAYS {
            board.push(match rng.below(10) {
                0..=2 => Sp::Mail,
                3..=4 => Sp::Deal,
                5 => Sp::Sell,
                _ => Sp::Plain,
            });
        }
        Salariu {
            board,
            pos: [0; 2],
            cash: [10; 2],
            lots: [0; 2],
            offer: false,
            side: Player::One,
            rng,
        }
    }
    fn net(&self, p: usize) -> i32 {
        self.cash[p] + 3 * self.lots[p]
    }
}

impl Default for Salariu {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Salariu {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        if self.offer {
            let mut v = vec!["las".to_string()];
            if self.cash[me] >= 5 {
                v.push("cumpar".into());
            }
            return v;
        }
        vec!["zar".into()]
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        if self.offer {
            if m == "cumpar" {
                self.cash[me] -= 5;
                self.lots[me] += 1;
            }
            self.offer = false;
            self.next_player();
            return;
        }
        let d = self.rng.d6() as usize;
        let old = self.pos[me];
        let np = (old + d).min(DAYS);
        // salarii traversate
        for day in old + 1..=np {
            if day % 30 == 0 {
                self.cash[me] += 8;
            }
        }
        self.pos[me] = np;
        if np < DAYS {
            match self.board[np] {
                Sp::Mail => {
                    let v = self.rng.below(11) as i32 - 6;
                    self.cash[me] = (self.cash[me] + v).max(0);
                }
                Sp::Sell => {
                    self.cash[me] += 9 * self.lots[me];
                    self.lots[me] = 0;
                }
                Sp::Deal => {
                    self.offer = true;
                    return;
                }
                Sp::Plain => {}
            }
        }
        self.next_player();
    }
    fn result(&self) -> Option<Outcome> {
        if self.pos[0] >= DAYS && self.pos[1] >= DAYS {
            return Some(by_score(self.net(0), self.net(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let f = |i: usize| self.cash[i] + 6 * self.lots[i];
        f(a) - f(1 - a)
    }
    fn show(&self) -> String {
        let mut s = String::from("Ziua de Salariu — poștă M, ofertă O, vânzare V, simplu .\n");
        for month in 0..3 {
            for d in 0..30 {
                let i = month * 30 + d;
                let mut ch = match self.board[i] {
                    Sp::Plain => '.',
                    Sp::Mail => 'M',
                    Sp::Deal => 'O',
                    Sp::Sell => 'V',
                };
                if (i + 1) % 30 == 0 {
                    ch = '$';
                }
                for p in 0..2 {
                    if self.pos[p] == i + 1 {
                        ch = if p == 0 { '1' } else { '2' };
                    }
                }
                s.push(ch);
            }
            s.push('\n');
        }
        for p in 0..2 {
            s.push_str(&format!(
                "J{}: ziua {} | bani {} | loturi {}\n",
                p + 1,
                self.pos[p],
                self.cash[p],
                self.lots[p]
            ));
        }
        s.push_str(&format!(
            "La mutare: {} ({})\n",
            pname(self.side),
            if self.offer { "cumpar | las" } else { "zar" }
        ));
        s
    }
}

impl Salariu {
    fn next_player(&mut self) {
        let other = 1 - pi(self.side);
        if self.pos[other] < DAYS {
            self.side = self.side.other();
        }
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Salariu::with_seed, 400);
    }
}
