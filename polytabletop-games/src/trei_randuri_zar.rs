//! Trei Rânduri cu Zaruri — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: o foaie cu 3 rânduri de 8 căsuțe. Cel la rând aruncă trei zaruri și alege care se adună (dados m); apoi amândoi scriu suma într-o căsuță (pun r c): crescător pe rând, fără număr repetat pe coloană. Cine nu poate sau nu vrea să scrie (pas) primește o penalizare dacă e la rând. Rând complet +6, coloană completă +3.
use crate::c5_rows::*;

pub static SPEC: RwSpec = RwSpec {
    title: "Trei Rânduri cu Zaruri",
    rows: 3,
    cols: 8,
    row_bonus: 6,
    col_bonus: 3,
    pen_max: 4,
    cap: 800,
};

pub fn new() -> Rw {
    Rw::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Rw::with_seed(&SPEC, s), 860);
    }
}
