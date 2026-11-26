#![allow(clippy::manual_is_multiple_of)]
//! Truth or Dare (Adevăr sau provocare) — joc de petrecere pentru 2 jucători, cu punctaj.
//!
//! În fiecare dintre cele 12 ture, jucătorul la rând alege `adevar` sau `provocare` (sau `refuz`, care costă 1 punct).
//! Jocul afișează întrebarea sau provocarea (familiale, fără conținut jenant); jucătorul o execută pe
//! viu, iar celălalt joacă rolul judecătorului: `ok` (executat cinstit: performerul primește 1 punct la adevăr, 2 la
//! provocare, iar judecătorul 1 punct pentru arbitraj corect) sau `nu` (performerul pierde 1 punct).
//! Textele sunt luate din liste fixe (fără noroc, fără informație ascunsă). Cel mai mare scor după
//! 12 ture câștigă; la egalitate remiză. AI-ul alege după punctaj (provocarea valorează dublu), iar în
//! rol de judecător acceptă întotdeauna executarea (nu poate verifica fapte reale).

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TURNS: u8 = 12;

const TRUTHS: [&str; 12] = [
    "Care este cea mai amuzantă amintire din copilăria ta?",
    "Ce mâncare nu ai suporta să mai mănânci niciodată?",
    "Ce superputere ți-ai dori și de ce?",
    "Care e cel mai ciudat vis pe care ți-l amintești?",
    "Ce film ai vedea de zece ori la rând?",
    "Care a fost cel mai bun cadou primit vreodată?",
    "Ce loc din lume ai vrea cel mai mult să vizitezi?",
    "Ce meserie ai fi ales dacă nu ai fi făcut ce faci acum?",
    "Care este cea mai mare frică mică a ta (păianjeni, înălțime...)?",
    "Ce obicei ciudat ai când ești singur acasă?",
    "Care este melodia pe care o cânți când crezi că nu te aude nimeni?",
    "Ce nume ai da unui animal de companie imaginar?",
];

const DARES: [&str; 12] = [
    "Imită un animal timp de zece secunde.",
    "Spune alfabetul pe dos cât poți de repede.",
    "Dansează fără muzică timp de cincisprezece secunde.",
    "Povestește o întâmplare inventată în exact trei propoziții.",
    "Vorbește cu accent străin până la următoarea ta tură.",
    "Fă zece genuflexiuni numărând cu voce tare.",
    "Cântă primul vers dintr-un cântec ales de adversar.",
    "Desenează cu degetul în aer un portret al adversarului.",
    "Spune un frământ de limbă de trei ori fără să greșești.",
    "Mergi într-un picior prin cameră și înapoi.",
    "Fă adversarului trei complimente originale.",
    "Mimează o meserie, iar adversarul trebuie să o ghicească.",
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Truth,
    Dare,
    Refuse,
    Ok,
    No,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Choose,
    Judge(bool),
    Over,
}

#[derive(Clone)]
pub struct TruthDare {
    turn: u8,
    scores: [i32; 2],
    phase: Phase,
    prompt: usize,
    log: String,
}

impl Default for TruthDare {
    fn default() -> Self {
        Self::new()
    }
}

impl TruthDare {
    pub fn new() -> Self {
        TruthDare {
            turn: 1,
            scores: [0; 2],
            phase: Phase::Choose,
            prompt: 0,
            log: String::new(),
        }
    }

    pub fn scores(&self) -> (i32, i32) {
        (self.scores[0], self.scores[1])
    }

    fn performer(&self) -> Player {
        if self.turn % 2 == 1 {
            Player::One
        } else {
            Player::Two
        }
    }

    fn advance(&mut self) {
        if self.turn >= TURNS {
            self.phase = Phase::Over;
        } else {
            self.turn += 1;
            self.phase = Phase::Choose;
        }
    }
}

impl Game for TruthDare {
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Judge(_) => self.performer().other(),
            _ => self.performer(),
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::Choose => vec![Move::Truth, Move::Dare, Move::Refuse],
            Phase::Judge(_) => vec![Move::Ok, Move::No],
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        let who = self.performer().idx();
        match (*m, self.phase) {
            (Move::Truth, _) => {
                self.prompt = (self.turn as usize * 5) % TRUTHS.len();
                self.phase = Phase::Judge(false);
            }
            (Move::Dare, _) => {
                self.prompt = (self.turn as usize * 7 + 3) % DARES.len();
                self.phase = Phase::Judge(true);
            }
            (Move::Refuse, _) => {
                self.scores[who] -= 1;
                self.log = format!("J{} refuză (-1)", who + 1);
                self.advance();
            }
            (Move::Ok, Phase::Judge(dare)) => {
                let pts = if dare { 2 } else { 1 };
                self.scores[who] += pts;
                self.scores[1 - who] += 1;
                self.log = format!("J{} +{}, judecătorul +1", who + 1, pts);
                self.advance();
            }
            (Move::No, Phase::Judge(_)) => {
                self.scores[who] -= 1;
                self.log = format!("J{} nu a convins (-1)", who + 1);
                self.advance();
            }
            _ => {}
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(match self.scores[0].cmp(&self.scores[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut v = (self.scores[p.idx()] - self.scores[p.other().idx()]) * 100;
        // o provocare în așteptare valorează mai mult pentru performer
        if let Phase::Judge(dare) = self.phase {
            let sign = if self.performer() == p { 1 } else { -1 };
            v += sign * if dare { 20 } else { 10 };
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Truth or Dare — tura {}/{} | scor: J1 {}, J2 {}\n",
            self.turn, TURNS, self.scores[0], self.scores[1]
        );
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        match self.phase {
            Phase::Over => s.push_str("Joc încheiat.\n"),
            Phase::Choose => s.push_str(&format!(
                "Jucătorul {}: alege «adevar», «provocare» sau «refuz».\n",
                self.performer().idx() + 1
            )),
            Phase::Judge(dare) => {
                s.push_str(&format!(
                    "{} pentru J{}: {}\n",
                    if dare { "Provocare" } else { "Adevăr" },
                    self.performer().idx() + 1,
                    if dare {
                        DARES[self.prompt]
                    } else {
                        TRUTHS[self.prompt]
                    }
                ));
                s.push_str(&format!(
                    "Judecător: Jucătorul {} («ok» sau «nu»)\n",
                    self.performer().other().idx() + 1
                ));
            }
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Truth => "adevar",
            Move::Dare => "provocare",
            Move::Refuse => "refuz",
            Move::Ok => "ok",
            Move::No => "nu",
        }
        .into()
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dare_ok_scores() {
        let mut g = TruthDare::new();
        g.apply(&Move::Dare);
        assert_eq!(g.to_move(), Player::Two);
        g.apply(&Move::Ok);
        assert_eq!(g.scores(), (2, 1));
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn refuse_costs_and_game_ends() {
        let mut g = TruthDare::new();
        for _ in 0..TURNS {
            g.apply(&Move::Refuse);
        }
        assert!(g.outcome().is_some());
        assert_eq!(g.scores(), (-6, -6));
        assert_eq!(g.outcome(), Some(Outcome::Draw));
    }

    #[test]
    fn prompts_exist_for_all_turns() {
        let mut g = TruthDare::new();
        for _ in 0..TURNS {
            g.apply(&Move::Truth);
            assert!(g.render().contains('?'));
            g.apply(&Move::Ok);
        }
        assert!(g.outcome().is_some());
    }
}
