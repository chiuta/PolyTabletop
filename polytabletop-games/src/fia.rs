#![allow(clippy::doc_lazy_continuation)]
//! Fia-med-knuff (Suedia), „ludo” suedez, în varianta pentru doi jucători: motorul comun
//! [`crate::ludofam`] cu regulile Fia.
//!
//! * Pista de 48 de pătrate, coloana de acasă de 4 pătrate, 4 piese pe jucător. O piesă iese din bază cu
//!   un **1 sau un 6**. Un 6 dă încă o aruncare. Aterizarea peste o piesă adversă o „knuffește” (o
//!   trimite în bază). Ținta se atinge doar cu zar exact. Câștigă cine are toate cele 4 piese la țintă.
//! * Simplificări: doi jucători (pe pătratele opuse), fără obligația de a lovi.
//! Mutări: `intra` sau numărul progresului piesei (ex. `12`).

use crate::ludofam::{LudoFam, Rules};

pub const RULES: Rules = Rules {
    name: "Fia-med-knuff",
    track: 48,
    home: 4,
    enter: &[1, 6],
    extra: &[6],
    safe_start: false,
};

pub fn new() -> LudoFam {
    LudoFam::new(RULES)
}

#[cfg(test)]
mod tests {
    #[test]
    fn variant_rules_hold() {
        crate::ludofam::tests::check_variant(super::RULES);
        assert!(super::RULES.enter.contains(&1) && super::RULES.enter.contains(&6));
    }
}
