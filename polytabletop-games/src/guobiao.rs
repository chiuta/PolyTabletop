//! Mahjong oficial chinez (Guobiao) — vezi `mj2.rs`.
use crate::mj2::{Mj2, Variant};

pub fn new() -> Mj2 {
    Mj2::new(Variant::Guobiao)
}
