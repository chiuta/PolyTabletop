//! Eroii din Arenă — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: Cavalerul contra Vrăjitoarei, 30 de viață, cinci zaruri cu simboluri proprii fiecărui erou, trei aruncări (arunc, reia m, gata). Apoi alegi o abilitate pe care o permit zarurile (abil k) sau nimic; simbolul „punct” strânge puncte de luptă pentru abilitatea supremă. Cine rămâne fără viață pierde.
use crate::c5_duel::*;

static CAVALER: Hero = Hero {
    name: "Cavalerul",
    syms: ["sabie", "scut", "inimă", "pas", "fulger", "punct"],
    abil: &[
        Ab {
            name: "Lovitură dreaptă",
            req: Req::Of(0, 3),
            dmg: 4,
            heal: 0,
            cp: 0,
        },
        Ab {
            name: "Lovitură dublă",
            req: Req::Of(0, 4),
            dmg: 7,
            heal: 0,
            cp: 0,
        },
        Ab {
            name: "Tăiere în cruce",
            req: Req::TwoPairs,
            dmg: 5,
            heal: 0,
            cp: 0,
        },
        Ab {
            name: "Scutul frăției",
            req: Req::Of(1, 3),
            dmg: 2,
            heal: 3,
            cp: 0,
        },
        Ab {
            name: "Marș forțat",
            req: Req::Run(4),
            dmg: 6,
            heal: 0,
            cp: 0,
        },
        Ab {
            name: "Furia cavalerului",
            req: Req::Of(0, 5),
            dmg: 14,
            heal: 0,
            cp: 4,
        },
    ],
};

static VRAJITOARE: Hero = Hero {
    name: "Vrăjitoarea",
    syms: ["steluță", "poțiune", "corb", "flacără", "vânt", "punct"],
    abil: &[
        Ab {
            name: "Rază de stele",
            req: Req::Of(0, 3),
            dmg: 3,
            heal: 2,
            cp: 0,
        },
        Ab {
            name: "Poțiune de leac",
            req: Req::Of(1, 3),
            dmg: 0,
            heal: 6,
            cp: 0,
        },
        Ab {
            name: "Corbul vestitor",
            req: Req::Of(2, 3),
            dmg: 5,
            heal: 0,
            cp: 0,
        },
        Ab {
            name: "Flacără vie",
            req: Req::Of(3, 4),
            dmg: 8,
            heal: 0,
            cp: 0,
        },
        Ab {
            name: "Vânt de furtună",
            req: Req::Run(4),
            dmg: 6,
            heal: 2,
            cp: 0,
        },
        Ab {
            name: "Furtuna stelară",
            req: Req::Of(0, 5),
            dmg: 14,
            heal: 4,
            cp: 4,
        },
    ],
};

pub static SPEC: DuelSpec = DuelSpec {
    title: "Eroii din Arenă",
    mode: DMode::Heroes,
    dice: 5,
    hp: 30,
    goal: 0,
    heroes: [&CAVALER, &VRAJITOARE],
    cap: 600,
};

pub fn new() -> Duel {
    Duel::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Duel::with_seed(&SPEC, s), 660);
    }
}
