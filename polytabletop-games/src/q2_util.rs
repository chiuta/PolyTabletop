//! Ajutoare comune pentru jocurile adăugate în valul 17 (Q2).

use polytabletop_core::{Outcome, Player};

/// Rezultat după un scor numeric (mai mare câștigă, egal = remiză).
pub fn by_score(a: i32, b: i32) -> Outcome {
    match a.cmp(&b) {
        std::cmp::Ordering::Greater => Outcome::Win(Player::One),
        std::cmp::Ordering::Less => Outcome::Win(Player::Two),
        std::cmp::Ordering::Equal => Outcome::Draw,
    }
}

/// Mică preferință de risc: în urmă cauți varianța (miza mare), în față o eviți.
pub fn risk(stake: i32, diff: i32) -> i32 {
    stake
        * if diff < 0 {
            12
        } else if diff > 0 {
            -12
        } else {
            0
        }
}

pub fn end_line(o: Option<Outcome>, side: Player) -> String {
    match o {
        Some(Outcome::Win(p)) => format!("Joc încheiat: câștigă J{}.\n", p.idx() + 1),
        Some(Outcome::Draw) => "Joc încheiat: remiză.\n".into(),
        None => format!("La mutare: J{}\n", side.idx() + 1),
    }
}
