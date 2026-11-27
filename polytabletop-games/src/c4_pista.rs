//! Două jocuri de deducție pentru copii (variante originale C4), pe motoarele comune `b6_ask` și `b6_deduce`.
//! * `vulpea_dibace`: un vulpoi ascuns dintre 12 suspecți (culoare, pălărie, coadă); ambii pun întrebări publice
//!   (`intreb A=v`) și ghicesc (`ghic k`); ținta nu o știe nimeni (clasa „fără informație privată”).
//! * `fantoma_blanda`: un fantomă prietenoasă (descrie) arată cel mult 2–3 indicii adevărate; celălalt ghicește
//!   (`ind A=v`, `gata`, `ghic k`, `verific`); ținta e văzută doar de cel care descrie (HOTSEAT_SAFE).
use crate::b6_ask::*;
use crate::b6_deduce::*;
use crate::brand_util::B;

static VULPE: ASpec = ASpec {
    title: "Vulpea Dibace",
    vals: &[4, 3, 2],
    mode: AMode::Shared,
    rounds: 4,
    qmax: 3,
    attr_names: &["culoare", "pălărie", "coadă"],
};

static FANTOMA: DSpec = DSpec {
    title: "Fantoma cea Blândă",
    vals: &[4, 3],
    caps: &[2, 3],
    mask: 3,
    rounds: 6,
    attr_names: &["vis", "culoare"],
};

pub fn make(id: &str) -> Box<dyn polytabletop_core::GameObj> {
    match id {
        "vulpea_dibace" => Box::new(B(Ask::new(&VULPE))),
        _ => Box::new(B(Deduce::new(&FANTOMA))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plays_ask() {
        crate::brand_util::check(|s| Ask::with_seed(&VULPE, s), 600);
    }

    #[test]
    fn plays_deduce() {
        crate::brand_util::check(|s| Deduce::with_seed(&FANTOMA, s), 600);
    }

    #[test]
    fn hidden_information_and_ai_honesty() {
        crate::b6_ask::audit(&VULPE);
        crate::b6_deduce::audit(&FANTOMA);
    }
}
