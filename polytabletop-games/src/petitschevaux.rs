#![allow(clippy::doc_lazy_continuation)]
//! Jeu des petits chevaux (Franța), cursa cu „căluți” și zar, în varianta pentru doi jucători: motorul
//! comun [`crate::ludofam`] cu regulile franceze.
//!
//! * Pista de 56 de pătrate (14 pe latură), „scara” (escalier) de acasă de 6 pătrate, 4 căluți pe jucător.
//!   Un căluț iese din grajd **doar cu un 6**; orice 6 dă încă o aruncare. Aterizarea pe căluțul
//!   adversar îl trimite în grajd, dar un căluț care stă pe propriul pătrat de start nu poate fi „mâncat”.
//!   Centrul (ținta) se atinge numai cu zar exact. Câștigă cine are toți cei 4 căluți în centru.
//! * Simplificări: doi jucători, fără pătrate-stea speciale și fără bariere formate din două piese.
//! Mutări: `intra` (din grajd) sau numărul progresului căluțului (ex. `12`).

use crate::ludofam::{LudoFam, Rules};

pub const RULES: Rules = Rules {
    name: "Jeu des petits chevaux",
    track: 56,
    home: 6,
    enter: &[6],
    extra: &[6],
    safe_start: true,
};

pub fn new() -> LudoFam {
    LudoFam::new(RULES)
}

#[cfg(test)]
mod tests {
    #[test]
    fn variant_rules_hold() {
        crate::ludofam::tests::check_variant(super::RULES);
    }
}
