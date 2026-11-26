//! Marcatorii din textul `render()` (în română) după care `board.rs` și `session.rs`
//! decupează tablele grafice și liniile de subsol. Un singur loc: dacă `render()` din
//! `polytabletop-games` se schimbă, testul `render_markers_present` pică și arată
//! exact ce marcator a dispărut. (Randarea nu e localizată: ține de limba sursă.)

/// Mancala și rudele: rândul „Jucător 2” (sus), apoi cel al lui „Jucător 1” (jos).
pub const PITS_P2: &str = "Jucător 2";
pub const PITS_P1: &str = "Jucător 1";
/// Shogi: începutul liniilor cu mâinile și cuvântul dinaintea pieselor din mână.
pub const SHOGI_GOTE: &str = "gote";
pub const SHOGI_SENTE: &str = "sente";
pub const SHOGI_HAND: &str = "mână:";
/// Table: linia barei/ieșirilor și zarurile rămase.
pub const BACKGAMMON_BAR: &str = "Bară";
pub const BACKGAMMON_DICE_LEFT: &str = "Rămase";
/// Puncte: linia cu scorul căsuțelor.
pub const PUNCTE_BOXES: &str = "Căsuțe";
/// Ludo/Pachisi: liniile jucătorilor („… (X) … bază N … [..] … terminate/acasă N”),
/// liniile de pistă / legendă (excluse din subsol) și zarul.
pub const RACE_BASE: &str = "bază";
pub const RACE_DONE: [&str; 2] = ["terminate", "acasă"];
pub const RACE_TRACK: &str = "Pista";
pub const RACE_LEGEND: &str = "X/O =";
pub const RACE_DICE: [&str; 2] = ["Zar:", "a aruncat"];
/// Coridor: linia cu zidurile rămase.
pub const CORRIDOR_WALLS: &str = "Ziduri";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::{kind_of, Kind};
    use polytabletop_core::rng::Rng;
    use polytabletop_games::playable;

    fn lines_of(r: &str) -> Vec<&str> {
        r.lines().map(str::trim_start).collect()
    }

    /// Marcatorii cerând prezență în randarea inițială (obligatorii) și, pe parcursul
    /// unei partide aleatoare, cei care apar doar în anumite stări (opționali).
    fn check_kind(kind: Kind, r: &str, seen_opt: &mut Vec<&'static str>) {
        let ls = lines_of(r);
        let starts = |m: &str| ls.iter().any(|l| l.starts_with(m));
        let has = |m: &str| r.contains(m);
        match kind {
            Kind::Pits => {
                assert!(starts(PITS_P2) && starts(PITS_P1), "pits:\n{r}");
            }
            Kind::Shogi => {
                assert!(
                    r.lines().any(|l| l.starts_with(SHOGI_GOTE))
                        && r.lines().any(|l| l.starts_with(SHOGI_SENTE)),
                    "shogi:\n{r}"
                );
                if has(SHOGI_HAND) {
                    seen_opt.push("shogi-hand");
                }
            }
            Kind::Backgammon => {
                assert!(r.lines().any(|l| l.starts_with(BACKGAMMON_BAR)), "{r}");
                if has(BACKGAMMON_DICE_LEFT) {
                    seen_opt.push("bg-dice");
                }
            }
            Kind::Puncte => {
                assert!(r.lines().any(|l| l.starts_with(PUNCTE_BOXES)), "{r}");
            }
            Kind::Ludo | Kind::Pachisi => {
                assert!(
                    r.lines()
                        .any(|l| l.contains(RACE_BASE) && l.contains("(X)")),
                    "{r}"
                );
                assert!(
                    r.lines()
                        .any(|l| l.contains(RACE_BASE) && l.contains("(O)")),
                    "{r}"
                );
                assert!(starts(RACE_TRACK), "{r}");
                if kind == Kind::Pachisi {
                    assert!(starts(RACE_LEGEND), "{r}");
                }
                if RACE_DONE.iter().any(|m| has(m)) {
                    seen_opt.push("race-done");
                }
                if RACE_DICE.iter().any(|m| has(m)) {
                    seen_opt.push("race-dice");
                }
            }
            Kind::Coridor => {
                assert!(r.lines().any(|l| l.starts_with(CORRIDOR_WALLS)), "{r}");
            }
            _ => {}
        }
    }

    /// Toate cele 36 de jocuri cu tablă grafică: randarea (inițială + o partidă scurtă
    /// aleatoare) conține marcatorii pe care îi folosesc parserele.
    #[test]
    fn render_markers_present() {
        let mut graphic = 0;
        let mut opt: Vec<&'static str> = Vec::new();
        for p in playable() {
            let Some(kind) = kind_of(p.id) else { continue };
            graphic += 1;
            let mut g = (p.make)();
            let mut rng = Rng::new(7);
            let mut n = 0;
            while g.outcome().is_none() && n < 120 {
                let r = g.render();
                check_kind(kind, &r, &mut opt);
                assert!(
                    crate::board::parse(kind, &r).is_some(),
                    "{}: parsare eșuată la ply {n}\n{r}",
                    p.id
                );
                let legal = g.legal_moves();
                let m = legal[rng.below(legal.len())].clone();
                g.play(&m).unwrap();
                n += 1;
            }
        }
        assert_eq!(graphic, 36, "numărul de jocuri cu tablă grafică");
        for want in ["shogi-hand", "bg-dice", "race-dice"] {
            assert!(
                opt.contains(&want),
                "marcator opțional niciodată văzut: {want}"
            );
        }
    }
}
