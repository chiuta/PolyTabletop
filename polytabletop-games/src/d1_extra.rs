//! Tărâmuri în Furtună — variantă originală de miniaturi (motorul `mn_engine`): două oști pe o tablă 8×8 cu terenuri grele
//! și ziduri, scop eliminarea adversarului (puncte pe pierderi). Reguli proprii, fără nume sau texte comerciale.
use crate::mn_engine::*;

pub static TARIMURI_IN_FURTUNA: MSpec = MSpec {
    title: "Tărâmuri în Furtună",
    w: 8,
    h: 8,
    hex: false,
    diag: true,
    map: "..~..~../.#....#./~......~/..~..~../..~..~../~......~/.#....#./..~..~..",
    uts: &[
        UT {
            sym: 'p',
            name: "Paladinul",
            hp: 4,
            mv: 3,
            rng: 1,
            dmg: 3,
            arm: 2,
            fl: "C",
        },
        UT {
            sym: 'f',
            name: "Furtunarul",
            hp: 2,
            mv: 3,
            rng: 4,
            dmg: 3,
            arm: 0,
            fl: "",
        },
        UT {
            sym: 'g',
            name: "Gardianul",
            hp: 5,
            mv: 2,
            rng: 1,
            dmg: 2,
            arm: 3,
            fl: "H",
        },
        UT {
            sym: 'z',
            name: "Zburătorul",
            hp: 2,
            mv: 5,
            rng: 1,
            dmg: 2,
            arm: 0,
            fl: "L",
        },
    ],
    army: ["pfgz", "pfgz"],
    goal: 0,
    act: 3,
    turns: 10,
    target: 0,
    fl: "dcf",
};

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn tarimuri_in_furtuna() {
        check_spec(&TARIMURI_IN_FURTUNA);
    }
}
