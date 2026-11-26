//! Hnefatafl 11×11 (Scandinavia, varianta Copenhaga) — vezi `tafl.rs`.
use crate::tafl::{Tafl, HNEFATAFL};

pub fn new() -> Tafl {
    Tafl::new(HNEFATAFL)
}
