//! Celebrity / Fishbowl (joc de petrecere), 2 echipe — fiecare echipă = un jucător + un coechipier simulat.
//!
//! * Vasul conține 12 nume de personalități (istorice sau fictive) alese la sorți din 20. Se joacă **trei runde** cu
//!   același vas: (1) **descrii** în cuvinte, (2) **mimezi**, (3) spui **un singur cuvânt**. Rundele au câte un
//!   indiciu pregătit pentru fiecare nume (se vede doar la mutarea ta).
//! * La rândul tău (3 încercări) vezi numele de sus: `dau` = dai indiciul rundei, iar coechipierul ghicește (șansă
//!   80% în runda 1, 65% în a doua, 50% în a treia, minus 10 puncte procentuale pentru fiecare nivel de dificultate
//!   peste 1); `sar` = pui numele la fundul vasului (și pierzi încercarea). Numele ghicite se scot din vas; ghicitul
//!   dă 1 punct. Runda se termină când vasul e gol; câștigă la final cine are mai multe puncte.
//! * Coechipierul ghicește în mutarea forțată `ghicesc`, deci AI-ul nu cunoaște rezultatul înainte să aleagă.
//!
//! Mutări: `dau`, `sar`, `ghicesc`. Informație ascunsă: numele de sus se arată doar celui care îl dă (`set_viewer`).

use crate::cards::{handover_banner, Viewer};
use crate::q6_util::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

/// (nume, descriere, mimă, un cuvânt, dificultate 1..3)
const NAMES: [(&str, &str, &str, &str, u8); 20] = [
    (
        "Napoleon Bonaparte",
        "Împărat francez scund, învins la Waterloo.",
        "Ții mâna în haină și pozezi ca un general.",
        "Waterloo",
        1,
    ),
    (
        "Leonardo da Vinci",
        "Pictor și inventator renascentist; a pictat o doamnă cu zâmbet misterios.",
        "Pictezi încet un portret zâmbind enigmatic.",
        "Gioconda",
        1,
    ),
    (
        "Cleopatra",
        "Regină a Egiptului, legată de Cezar și Marc Antoniu.",
        "Te prefaci regină cu un șarpe în mână.",
        "Egipt",
        1,
    ),
    (
        "Albert Einstein",
        "Fizician cu părul vâlvoi, autorul teoriei relativității.",
        "Scoți limba și scrii o formulă în aer.",
        "Relativitate",
        1,
    ),
    (
        "Mihai Eminescu",
        "Poetul național al românilor, autorul Luceafărului.",
        "Recitezi patetic o poezie privind spre stele.",
        "Luceafărul",
        1,
    ),
    (
        "Vlad Țepeș",
        "Domnitor al Țării Românești, cunoscut pentru pedepsele cu țeapa.",
        "Ții o suliță lungă cu privirea aspră.",
        "Țeapă",
        2,
    ),
    (
        "Charlie Chaplin",
        "Actor de film mut, cu mustață mică, pălărie și baston.",
        "Mergi legănat, cu un baston imaginar.",
        "Vagabondul",
        2,
    ),
    (
        "Marie Curie",
        "Savantă de două ori laureată Nobel, descoperitoarea radiului.",
        "Privești încântat un tub de sticlă care strălucește.",
        "Radiu",
        2,
    ),
    (
        "Cristofor Columb",
        "Navigator care a ajuns în America în 1492.",
        "Privești cu luneta spre orizont, de pe o corabie.",
        "America",
        1,
    ),
    (
        "William Shakespeare",
        "Dramaturg englez, autorul lui Hamlet și Romeo și Julieta.",
        "Ții un craniu și spui «a fi sau a nu fi».",
        "Hamlet",
        1,
    ),
    (
        "Wolfgang Amadeus Mozart",
        "Compozitor austriac, copil-minune, autorul lui Requiem.",
        "Dirijezi cu o perucă imaginară.",
        "Requiem",
        2,
    ),
    (
        "Nikola Tesla",
        "Inventator sârb-american al curentului alternativ.",
        "Faci fulgere între degete.",
        "Bobină",
        2,
    ),
    (
        "Sherlock Holmes",
        "Detectiv fictiv din strada Baker, cu pipă și lupă.",
        "Cercetezi o urmă cu lupa, tras din pipă.",
        "Elementar",
        1,
    ),
    (
        "Dracula",
        "Conte vampir din Transilvania.",
        "Arăți colții și ridici gulerul pelerinei.",
        "Vampir",
        1,
    ),
    (
        "Don Quijote",
        "Cavaler rătăcitor care se luptă cu morile de vânt.",
        "Te lupți călare cu o moară de vânt imaginară.",
        "Moară",
        2,
    ),
    (
        "Robin Hood",
        "Haiduc englez din pădurea Sherwood, care fură de la bogați.",
        "Tragi cu arcul într-o țintă îndepărtată.",
        "Arc",
        1,
    ),
    (
        "Ștefan cel Mare",
        "Domnitor al Moldovei care a ridicat multe mănăstiri.",
        "Ridici sabia, apoi te închini smerit.",
        "Putna",
        2,
    ),
    (
        "Constantin Brâncuși",
        "Sculptor român, autorul Coloanei fără sfârșit.",
        "Cioplești o piatră și o lustruiești.",
        "Coloană",
        3,
    ),
    (
        "George Enescu",
        "Compozitor și violonist român, autorul Rapsodiilor române.",
        "Cânți înflăcărat la vioară.",
        "Rapsodie",
        2,
    ),
    (
        "Isaac Newton",
        "Fizician care a formulat gravitația după un măr căzut.",
        "Stai sub un copac și îți cade ceva în cap.",
        "Gravitație",
        1,
    ),
];

const BOWL: usize = 12;
const TRIES: u8 = 3;
const CHANCE: [i32; 3] = [80, 65, 50];

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    Give,
    Skip,
    Guess,
}

#[derive(Clone)]
pub struct Celebrity {
    rng: Rng,
    names: Vec<u8>,
    bowl: Vec<u8>,
    round: usize,
    side: Player,
    tries: u8,
    pending: bool,
    score: [i32; 2],
    log: String,
    viewer: Viewer,
    prev: Player,
}

impl Default for Celebrity {
    fn default() -> Self {
        Self::new()
    }
}

impl Celebrity {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut all: Vec<u8> = (0..NAMES.len() as u8).collect();
        shuffle(&mut all, &mut rng);
        let names: Vec<u8> = all[..BOWL].to_vec();
        let mut bowl = names.clone();
        shuffle(&mut bowl, &mut rng);
        Celebrity {
            rng,
            names,
            bowl,
            round: 0,
            side: Player::One,
            tries: 0,
            pending: false,
            score: [0, 0],
            log: String::new(),
            viewer: Viewer::Fixed(Player::One),
            prev: Player::One,
        }
    }

    pub fn scores(&self) -> [i32; 2] {
        self.score
    }

    fn top(&self) -> usize {
        *self.bowl.last().unwrap() as usize
    }

    fn chance(&self) -> i32 {
        CHANCE[self.round] - 10 * (NAMES[self.top()].4 as i32 - 1)
    }

    fn over(&self) -> bool {
        self.round >= 3
    }

    fn next_turn(&mut self) {
        self.tries = 0;
        self.side = self.side.other();
    }
}

impl Game for Celebrity {
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
        if self.over() {
            return Vec::new();
        }
        if self.pending {
            vec![Move::Guess]
        } else {
            vec![Move::Give, Move::Skip]
        }
    }

    fn apply(&mut self, m: &Move) {
        self.prev = self.side;
        let me = self.side.idx();
        match m {
            Move::Give => {
                self.pending = true;
            }
            Move::Skip => {
                let c = self.bowl.pop().unwrap();
                self.bowl.insert(0, c);
                self.log = format!("J{} sare un nume.", me + 1);
                self.tries += 1;
                if self.tries >= TRIES {
                    self.next_turn();
                }
            }
            Move::Guess => {
                self.pending = false;
                let ok = (self.rng.below(100) as i32) < self.chance();
                let c = self.top();
                if ok {
                    self.bowl.pop();
                    self.score[me] += 1;
                    self.log = format!("J{}: coechipierul a ghicit «{}» (+1).", me + 1, NAMES[c].0);
                } else {
                    let c = self.bowl.pop().unwrap();
                    self.bowl.insert(0, c);
                    self.log = format!(
                        "J{}: coechipierul n-a ghicit, numele merge la fundul vasului.",
                        me + 1
                    );
                }
                self.tries += 1;
                if self.bowl.is_empty() {
                    self.round += 1;
                    if self.round < 3 {
                        self.bowl = self.names.clone();
                        shuffle(&mut self.bowl, &mut self.rng);
                        self.log
                            .push_str(&format!(" Începe runda {}.", self.round + 1));
                        self.next_turn();
                    }
                } else if self.tries >= TRIES {
                    self.next_turn();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over() {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut v = (self.score[p.idx()] - self.score[p.other().idx()]) * 100;
        if !self.over() && self.side == p && !self.bowl.is_empty() {
            let q = self.chance();
            if self.pending {
                v += q;
            } else {
                // sărirea valorează cât o șansă medie
                v += 55 - 10 * (self.round as i32);
            }
        }
        v
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if self.prev != self.side && !self.over() && !self.pending {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        s.push_str(&format!(
            "Celebrity — runda {}/3 ({}), în vas: {} nume; scor J1 {} – J2 {}\n",
            (self.round + 1).min(3),
            ["descrii", "mimezi", "un singur cuvânt"][self.round.min(2)],
            self.bowl.len(),
            self.score[0],
            self.score[1]
        ));
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!("Jucătorul {} câștigă.\n", p.idx() + 1)),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => {
                let v = self.viewer.who(self.side);
                if v == self.side && !self.bowl.is_empty() {
                    let c = self.top();
                    let hint = [NAMES[c].1, NAMES[c].2, NAMES[c].3][self.round];
                    s.push_str(&format!(
                        "Cartea ta (J{}): {} — indiciu: {}\n",
                        v.idx() + 1,
                        NAMES[c].0,
                        hint
                    ));
                } else {
                    s.push_str(&format!(
                        "Cartea lui J{}: ascunsă (o vede doar cel care dă indiciile)\n",
                        self.side.idx() + 1
                    ));
                }
                if self.pending {
                    s.push_str("Coechipierul ghicește: scrie «ghicesc».\n");
                }
                s.push_str(&format!(
                    "La mutare: Jucătorul {} (încercarea {}/{TRIES})\n",
                    self.side.idx() + 1,
                    self.tries + 1
                ));
            }
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Give => "dau".into(),
            Move::Skip => "sar".into(),
            Move::Guess => "ghicesc".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn games_end_after_three_rounds() {
        for seed in 1..=10u64 {
            let mut g = Celebrity::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 5000);
            }
            assert_eq!(g.round, 3);
            assert_eq!(g.score[0] + g.score[1], 36);
        }
    }

    #[test]
    fn name_hidden_from_other_viewer() {
        let mut g = Celebrity::with_seed(3);
        g.set_viewer(Some(Player::Two));
        let r = g.render();
        let c = g.top();
        assert!(!r.contains(NAMES[c].0));
        g.set_viewer(Some(Player::One));
        assert!(g.render().contains(NAMES[c].0));
    }
}
