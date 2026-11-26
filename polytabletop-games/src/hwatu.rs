//! Hwatu (Go-Stop, Coreea) — 2 jucători; mecanismul e în `hanafuda.rs` (varianta `GoStop`).
//!
//! Reguli folosite (variantă documentată, din regulile publice ale Go-Stop pentru doi):
//! * Pachetul de 48 de cărți de flori; 10 cărți fiecare, 8 pe masă, 20 în stoc. O tură: joci o
//!   carte din mână (captură obligatorie dacă are pereche pe masă; doi candidați = alegi, trei =
//!   îi iei pe toți), apoi `intoarce` din stoc și joci la fel (`ia 3c` la alegere).
//! * Punctaj: Gwang (luminoase) — 3 = 3 puncte (2 dacă include omul-ploii), 4 = 4, 5 = 15;
//!   Godori (privighetoare + cuc + gâște) 5; Hongdan (3 panglici cu poem) 3; Cheongdan (3
//!   albastre) 3; Chodan (3 panglici roșii simple: aprilie, mai, iulie) 3; Tti (5+ panglici)
//!   1 + 1 pentru fiecare în plus; Yeolkkeut (5+ animale) 1 + 1 în plus; Pi (10+ „simple”,
//!   unde două cărți — o Paulownia și salcia — sunt „duble”) 1 + 1 în plus.
//! * Când ajungi la 7 puncte după o tură alegi `stop` (încasezi) sau `go` (continui). La
//!   fiecare nou scor mai mare decât la ultimul go ai din nou alegerea. Plata: baza + 1 pentru
//!   primul și al doilea go; de la al treilea go punctele se dublează de fiecare dată; ×2
//!   gwang-bak (câștigi cu 3+ luminoase, adversarul 0), ×2 pi-bak (ai 10+ simple, adversarul
//!   1–5), ×2 go-bak (adversarul care a spus go e învins).
//! * „Măturare” (masa golită printr-o captură) îți aduce o carte simplă de la adversar.
//! * Meci de 3 runde, câștigă cine are mai multe puncte. Mâinile golite fără `stop` = remiză
//!   în acea rundă.
//! * Simplificări: fără ppeok, jjok, ttadak, bombă și shaking; cupa de sake e doar animal;
//!   fără „meongtta”; AI-ul spune `stop` imediat ce ajunge la 7.

use crate::hanafuda::Hanafuda;

/// Un joc nou de Go-Stop (seed din ceas).
pub fn new() -> Hanafuda {
    Hanafuda::go_stop()
}

/// Un joc nou de Go-Stop cu seed fix (teste).
pub fn with_seed(seed: u64) -> Hanafuda {
    Hanafuda::go_stop_with_seed(seed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hanafuda::Variant;
    use polytabletop_core::{Game, Player};

    #[test]
    fn go_stop_variant_basics() {
        let g = with_seed(2);
        assert_eq!(g.variant(), Variant::GoStop);
        assert_eq!(g.to_move(), Player::One);
        let r = Game::render(&g);
        assert!(r.starts_with("Hwatu (Go-Stop)"));
        assert!(r.contains("J2: 10 cărți"));
    }
}
