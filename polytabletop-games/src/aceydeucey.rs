#![allow(clippy::doc_lazy_continuation)]
//! Acey-deucey (SUA, jocul de tablă al marinarilor), variantă de table pentru doi jucători, pe baza motorului
//! [`crate::bgfam`].
//!
//! * Toate cele 15 piese ale fiecărui jucător pornesc în afara tablei; intri pe tabla adversarului și mergi
//!   spre tabla ta (sensuri opuse, ca la table). Piesele pot fi mișcate în orice ordine, chiar dacă altele n-au
//!   intrat încă. O piesă singură e „dată afară” și reintră prin aceeași poartă.
//! * Dubla = patru mutări cu numărul respectiv, apoi arunci din nou. **Acey-deucey** (1 și 2): joci 1 și 2,
//!   apoi **alegi** orice număr 1–6 pe care îl joci ca dublet (de patru ori), apoi arunci din nou (`dublet N`).
//! * O piesă ajunsă în ultimul sfert nu mai poate fi mișcată decât ca să iasă, iar scoaterea cere
//!   zar exact și numai după ce toate cele 15 piese sunt în ultimul sfert. Câștigă cine scoate primul piesele.
//! * Regula blocajelor din jocul original (interzicerea unui „prim” de 6 puncte) nu e implementată.
//! Mutări: `ent-3`, `5-9`, `20-off`, `dublet 4`.

use crate::bgfam::{BgFam, Rules};

pub const RULES: Rules = Rules {
    name: "Acey-deucey",
    dice: 2,
    opposite: true,
    exact_off: true,
    home_locked: true,
    acey: true,
};

pub fn new() -> BgFam {
    BgFam::new(RULES)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bgfam::Mv;
    use polytabletop_core::Game;

    #[test]
    fn variant_rules_hold() {
        crate::bgfam::tests::check_variant(RULES);
    }

    #[test]
    fn acey_deucey_roll_lets_you_pick_a_doublet() {
        let mut found = false;
        for seed in 1..500u64 {
            let mut g = BgFam::with_seed(RULES, seed);
            // aruncarea inițială 1-2 → joacă cele două zaruri, apoi urmează alegerea
            let mut steps = 0;
            while steps < 3 {
                let mv = g.legal_moves();
                if mv.iter().any(|m| matches!(m, Mv::Choose(_))) {
                    assert_eq!(mv.len(), 6);
                    found = true;
                    break;
                }
                g.apply(&mv[0]);
                steps += 1;
            }
            if found {
                break;
            }
        }
        assert!(found);
    }
}
