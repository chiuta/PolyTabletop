#![allow(clippy::doc_lazy_continuation)]
//! Tabula (Roma antică, sec. V–VI d.Hr.), strămoșul tablelor, în reconstituirea publică din
//! epigrama împăratului Zenon, pentru doi jucători, pe baza motorului [`crate::bgfam`].
//!
//! * 24 de puncte, 15 piese pe jucător, toate pornesc în afara tablei; **trei zaruri** — fiecare zar
//!   mută o piesă (sau intră pe tablă) cu atâția pași. Ambii jucători merg în **același sens**, de la
//!   punctul 1 la 24, și intră pe punctele 1–6.
//! * Nu poți ateriza pe un punct ocupat de două sau mai multe piese adverse; o piesă adversă singură e
//!   lovită și trebuie reintrodusă.
//! * Sursele nu spun cum se scot piesele: alegem ca la table — scoți când toate cele 15 sunt în ultimul sfert
//!   (puncte 19–24), cu zar exact sau mai mare dacă nu mai ai piese în urmă. Câștigă cine scoate primul piesele.
//!   Obligația de reintrare nu e cunoscută; aici piesele pot fi mișcate în orice ordine.
//! Mutări: `ent-3`, `5-9`, `20-off`.

use crate::bgfam::{BgFam, Rules};

pub const RULES: Rules = Rules {
    name: "Tabula",
    dice: 3,
    opposite: false,
    exact_off: false,
    home_locked: false,
    acey: false,
};

pub fn new() -> BgFam {
    BgFam::new(RULES)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_rules_hold() {
        crate::bgfam::tests::check_variant(RULES);
    }

    #[test]
    fn three_dice_are_rolled_each_turn() {
        let g = BgFam::with_seed(RULES, 3);
        assert!(polytabletop_core::Game::render(&g).contains("Zaruri rămase: ["));
    }
}
