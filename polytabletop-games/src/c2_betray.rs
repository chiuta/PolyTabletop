//! Motor „explorare, apoi trădare” (lista C2): două faze. Faza 1: cei doi jucători (J1, J2)
//! explorează pe rând un pachet de camere (`explora`, apoi `revela` forțat) și își adună obiecte;
//! când se strâng destule semne începe apariția: unul dintre ei devine trădătorul. Faza 2 este un
//! duel cu informație perfectă pe o linie: eroul vrea să ajungă la ieșire sau să doboare creatura,
//! trădătorul vrea să-l doboare pe erou.
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct BSpec {
    pub title: &'static str,
    pub rooms: usize,
    pub omens: u32,
    pub len: i32,
    pub beast_hp: i32,
    pub hero_hp: i32,
    pub reveal_betrays: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Ph {
    Explore,
    Reveal,
    Duel,
}

#[derive(Clone)]
pub struct Bt {
    sp: &'static BSpec,
    deck: Vec<u8>,
    items: [i32; 2],
    omens: u32,
    ph: Ph,
    side: Player,
    traitor: usize,
    hp: i32,
    pos: i32,
    beast: i32,
    bhp: i32,
    shield: bool,
    stun: bool,
    plies: u32,
    winner: Option<Player>,
}

impl Bt {
    pub fn new(sp: &'static BSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static BSpec, seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        // 0 gol, 1 obiect, 2 semn (+obiect), 3 eveniment (+1 viață eroului viitor: contează ca obiect)
        let mut deck: Vec<u8> = (0..sp.rooms).map(|i| [0u8, 1, 2, 1, 2, 3][i % 6]).collect();
        shuf(&mut deck, &mut rng);
        Bt {
            sp,
            deck,
            items: [0, 0],
            omens: 0,
            ph: Ph::Explore,
            side: Player::One,
            traitor: 0,
            hp: 0,
            pos: 0,
            beast: 0,
            bhp: 0,
            shield: false,
            stun: false,
            plies: 0,
            winner: None,
        }
    }
    fn hero(&self) -> usize {
        1 - self.traitor
    }
    fn begin(&mut self, revealer: usize) {
        self.traitor = if self.sp.reveal_betrays {
            revealer
        } else {
            1 - revealer
        };
        let h = self.hero();
        self.hp = self.sp.hero_hp + self.items[h] / 2;
        self.bhp = self.sp.beast_hp + self.items[self.traitor] + self.omens as i32 / 2;
        self.pos = 0;
        self.beast = self.sp.len / 2 + 1;
        self.shield = false;
        self.stun = false;
        self.ph = Ph::Duel;
        self.side = crate::b6_util::pl(h);
    }
    fn dmg(&self) -> i32 {
        1 + self.omens as i32 / 2
    }
    fn dist(&self) -> i32 {
        (self.beast - self.pos).abs()
    }
}

impl Mini for Bt {
    fn side(&self) -> Player {
        self.side
    }
    fn depth(&self) -> u32 {
        if self.ph == Ph::Duel {
            4
        } else {
            0
        }
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Explore => vec!["explora".into()],
            Ph::Reveal => vec!["revela".into()],
            Ph::Duel => {
                let me = pi(self.side);
                let mut v = Vec::new();
                if me == self.hero() {
                    if !self.stun {
                        v.push("av".into());
                        if self.items[me] > 0 {
                            v.push("sprint".into());
                        }
                    }
                    if self.dist() <= 1 {
                        v.push("lovesc".into());
                        if self.items[me] > 0 {
                            v.push("lovesc+".into());
                        }
                    }
                    if self.items[me] > 0 && !self.shield {
                        v.push("scut".into());
                    }
                    v.push("pas".into());
                } else {
                    v.push("mers".into());
                    if self.beast < self.sp.len {
                        v.push("sare".into());
                    }
                    if self.dist() <= 1 {
                        v.push("izbeste".into());
                    }
                    if self.items[me] > 0 && !self.stun {
                        v.push("blestem".into());
                    }
                }
                v
            }
        }
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        self.plies += 1;
        match m {
            "explora" => self.ph = Ph::Reveal,
            "revela" => {
                let c = self.deck.pop().unwrap_or(2);
                match c {
                    1 | 3 => self.items[me] += 1,
                    2 => {
                        self.items[me] += 1;
                        self.omens += 1;
                    }
                    _ => {}
                }
                if self.omens >= self.sp.omens || self.deck.is_empty() {
                    self.begin(me);
                } else {
                    self.ph = Ph::Explore;
                    self.side = self.side.other();
                }
            }
            "av" => {
                self.pos += 1;
                self.side = self.side.other();
            }
            "sprint" => {
                self.items[me] -= 1;
                self.pos += 2;
                self.side = self.side.other();
            }
            "lovesc" | "lovesc+" => {
                let mut d = 1;
                if m == "lovesc+" {
                    self.items[me] -= 1;
                    d = 2;
                }
                self.bhp -= d;
                self.side = self.side.other();
            }
            "scut" => {
                self.items[me] -= 1;
                self.shield = true;
                self.side = self.side.other();
            }
            "pas" => self.side = self.side.other(),
            "mers" => {
                self.stun = false;
                self.beast += if self.beast > self.pos { -1 } else { 1 };
                self.side = self.side.other();
            }
            "sare" => {
                self.stun = false;
                self.beast = (self.beast + 1).min(self.sp.len);
                self.side = self.side.other();
            }
            "izbeste" => {
                self.stun = false;
                if self.shield {
                    self.shield = false;
                } else {
                    self.hp -= self.dmg();
                }
                self.side = self.side.other();
            }
            _ => {
                self.items[me] -= 1;
                self.stun = true;
                self.side = self.side.other();
            }
        }
        if self.ph == Ph::Duel {
            let h = self.hero();
            if self.pos >= self.sp.len || self.bhp <= 0 {
                self.winner = Some(crate::b6_util::pl(h));
            } else if self.hp <= 0 || self.plies > 70 {
                self.winner = Some(crate::b6_util::pl(self.traitor));
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        self.winner.map(Outcome::Win)
    }
    fn eval(&self, p: Player) -> i32 {
        let me = pi(p);
        if self.ph != Ph::Duel {
            return (self.items[me] - self.items[1 - me]) * 2;
        }
        let h = self.pos * 4 + self.hp * 5 - self.bhp * 4 + (self.shield as i32) * 3
            - (self.stun as i32) * 3
            + self.items[self.hero()] * 2
            - self.items[self.traitor] * 2;
        if me == self.hero() {
            h
        } else {
            -h
        }
    }
    fn show(&self) -> String {
        let mut s = format!("{} — ", self.sp.title);
        if self.ph == Ph::Duel {
            let h = self.hero();
            s.push_str(&format!(
                "apariția: J{} e erou, J{} e trădător\n  linia 0..{}: erou la {} (viață {}{}{}), creatura la {} (rezistență {}, lovitură {})\n  obiecte: J1 {}, J2 {}\n",
                h + 1, self.traitor + 1, self.sp.len, self.pos, self.hp,
                if self.shield { ", scut" } else { "" },
                if self.stun { ", paralizat" } else { "" },
                self.beast, self.bhp, self.dmg(), self.items[0], self.items[1]
            ));
            s.push_str(&format!(
                "La mutare: {} — erou: av | sprint | lovesc | lovesc+ | scut | pas; trădător: mers | sare | izbeste | blestem\n",
                pname(self.side)
            ));
        } else {
            s.push_str(&format!(
                "explorare: camere rămase {}, semne {}/{}\n  obiecte: J1 {}, J2 {}\nLa mutare: {} — explora / revela\n",
                self.deck.len(), self.omens, self.sp.omens, self.items[0], self.items[1], pname(self.side)
            ));
        }
        s
    }
}
