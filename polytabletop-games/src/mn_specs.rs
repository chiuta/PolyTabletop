//! Specificații (lista T3): miniaturi, variante proprii. Generat de work17/t3gen/gen.py.
use crate::mn_engine::*;

pub static EROI_PE_ACOPERIS: MSpec = MSpec {
    title: "Eroi pe acoperiș",
    w: 8,
    h: 8,
    hex: false,
    diag: true,
    map: "......../......../.....~#./.~**~.../...~**~./.#~...../......../........",
    uts: &[
        UT {
            sym: 'f',
            name: "Zburătorul",
            hp: 3,
            mv: 4,
            rng: 1,
            dmg: 2,
            arm: 1,
            fl: "L",
        },
        UT {
            sym: 't',
            name: "Titanul",
            hp: 5,
            mv: 2,
            rng: 1,
            dmg: 3,
            arm: 2,
            fl: "KH",
        },
        UT {
            sym: 'a',
            name: "Arcașa",
            hp: 2,
            mv: 3,
            rng: 4,
            dmg: 2,
            arm: 0,
            fl: "",
        },
        UT {
            sym: 's',
            name: "Scutierul",
            hp: 4,
            mv: 3,
            rng: 1,
            dmg: 2,
            arm: 2,
            fl: "",
        },
    ],
    army: ["ftas", "ftas"],
    goal: 1,
    act: 3,
    turns: 9,
    target: 9,
    fl: "dp",
};
pub static RAZBOINICI_DE_FIORD: MSpec = MSpec {
    title: "Războinici de fiord",
    w: 8,
    h: 6,
    hex: false,
    diag: false,
    map: "......../......../#..~~.../...~~..#/......../........",
    uts: &[
        UT {
            sym: 'g',
            name: "Garda",
            hp: 2,
            mv: 3,
            rng: 1,
            dmg: 3,
            arm: 1,
            fl: "F",
        },
        UT {
            sym: 'b',
            name: "Berserkul",
            hp: 3,
            mv: 4,
            rng: 1,
            dmg: 4,
            arm: 0,
            fl: "C",
        },
        UT {
            sym: 'l',
            name: "Lăncierul",
            hp: 2,
            mv: 3,
            rng: 2,
            dmg: 2,
            arm: 1,
            fl: "",
        },
        UT {
            sym: 'a',
            name: "Arcașul",
            hp: 1,
            mv: 3,
            rng: 4,
            dmg: 2,
            arm: 0,
            fl: "",
        },
    ],
    army: ["ggbla", "gglla"],
    goal: 0,
    act: 4,
    turns: 10,
    target: 0,
    fl: "dxf",
};
pub static OSTI_PE_O_FILA: MSpec = MSpec {
    title: "Oști pe o filă",
    w: 9,
    h: 9,
    hex: false,
    diag: true,
    map:
        "........./........./..~.....#/......~.~/...#.#.../~.~....../#.....~../........./.........",
    uts: &[
        UT {
            sym: 'p',
            name: "Pușcaș",
            hp: 2,
            mv: 3,
            rng: 5,
            dmg: 2,
            arm: 1,
            fl: "",
        },
        UT {
            sym: 'm',
            name: "Mercenar",
            hp: 3,
            mv: 3,
            rng: 1,
            dmg: 3,
            arm: 1,
            fl: "",
        },
        UT {
            sym: 'o',
            name: "Ogar",
            hp: 5,
            mv: 2,
            rng: 1,
            dmg: 4,
            arm: 2,
            fl: "H",
        },
        UT {
            sym: 'r',
            name: "Cercetaș",
            hp: 1,
            mv: 5,
            rng: 3,
            dmg: 1,
            arm: 0,
            fl: "L",
        },
    ],
    army: ["ppmmor", "pppmor"],
    goal: 0,
    act: 3,
    turns: 9,
    target: 16,
    fl: "dr",
};
pub static BANDE_IN_RUINE: MSpec = MSpec {
    title: "Bande în ruine",
    w: 8,
    h: 8,
    hex: false,
    diag: false,
    map: "......../......../.#~#~~~./.~#.~.../...~.#~./.~~~#~#./......../........",
    uts: &[
        UT {
            sym: 'c',
            name: "Căpitan",
            hp: 3,
            mv: 3,
            rng: 1,
            dmg: 3,
            arm: 1,
            fl: "F",
        },
        UT {
            sym: 'h',
            name: "Hoț",
            hp: 2,
            mv: 4,
            rng: 1,
            dmg: 2,
            arm: 0,
            fl: "",
        },
        UT {
            sym: 't',
            name: "Tragător",
            hp: 2,
            mv: 3,
            rng: 4,
            dmg: 2,
            arm: 0,
            fl: "",
        },
        UT {
            sym: 'd',
            name: "Doctor",
            hp: 2,
            mv: 3,
            rng: 1,
            dmg: 1,
            arm: 0,
            fl: "M",
        },
    ],
    army: ["chhtd", "chhtd"],
    goal: 0,
    act: 5,
    turns: 10,
    target: 0,
    fl: "dv",
};
pub static MASINARII_DE_ABUR: MSpec = MSpec {
    title: "Mașinării de abur",
    w: 8,
    h: 8,
    hex: false,
    diag: true,
    map: "......../......../#~....../~...#.~./.~.#...~/......~#/......../........",
    uts: &[
        UT {
            sym: 'k',
            name: "Colos",
            hp: 6,
            mv: 2,
            rng: 1,
            dmg: 4,
            arm: 3,
            fl: "HB",
        },
        UT {
            sym: 't',
            name: "Tunar",
            hp: 3,
            mv: 2,
            rng: 5,
            dmg: 3,
            arm: 1,
            fl: "B",
        },
        UT {
            sym: 's',
            name: "Soldat",
            hp: 2,
            mv: 3,
            rng: 1,
            dmg: 2,
            arm: 1,
            fl: "",
        },
        UT {
            sym: 'm',
            name: "Mecanic",
            hp: 2,
            mv: 3,
            rng: 1,
            dmg: 1,
            arm: 0,
            fl: "M",
        },
    ],
    army: ["kssmt", "kssmt"],
    goal: 0,
    act: 3,
    turns: 9,
    target: 0,
    fl: "dr",
};
pub static STRIGOI_DE_TARG: MSpec = MSpec {
    title: "Strigoi de târg",
    w: 7,
    h: 7,
    hex: false,
    diag: true,
    map: "......./......./#~...../~*.*.*~/.....~#/......./.......",
    uts: &[
        UT {
            sym: 'f',
            name: "Fantomă",
            hp: 2,
            mv: 4,
            rng: 1,
            dmg: 2,
            arm: 0,
            fl: "L",
        },
        UT {
            sym: 'l',
            name: "Lampadar",
            hp: 3,
            mv: 3,
            rng: 3,
            dmg: 2,
            arm: 1,
            fl: "",
        },
        UT {
            sym: 'p',
            name: "Păpușar",
            hp: 3,
            mv: 3,
            rng: 1,
            dmg: 3,
            arm: 1,
            fl: "K",
        },
        UT {
            sym: 'c',
            name: "Clovn",
            hp: 2,
            mv: 4,
            rng: 1,
            dmg: 3,
            arm: 0,
            fl: "C",
        },
    ],
    army: ["fflpc", "fflpc"],
    goal: 1,
    act: 3,
    turns: 9,
    target: 8,
    fl: "dpx",
};
pub static GIGANTI_DE_OTEL: MSpec = MSpec {
    title: "Giganți de oțel",
    w: 9,
    h: 8,
    hex: true,
    diag: false,
    map: "........./........./.#..~#~../...#~.~~./.~~.~#.../..~#~..#./........./.........",
    uts: &[
        UT {
            sym: 'u',
            name: "Uriaș",
            hp: 6,
            mv: 2,
            rng: 5,
            dmg: 4,
            arm: 2,
            fl: "H",
        },
        UT {
            sym: 'm',
            name: "Mijlociu",
            hp: 4,
            mv: 3,
            rng: 4,
            dmg: 3,
            arm: 1,
            fl: "H",
        },
        UT {
            sym: 's',
            name: "Sprinter",
            hp: 2,
            mv: 5,
            rng: 2,
            dmg: 2,
            arm: 0,
            fl: "L",
        },
    ],
    army: ["umss", "umss"],
    goal: 0,
    act: 4,
    turns: 10,
    target: 0,
    fl: "dr",
};
pub static SALVE_SI_BAIONETE: MSpec = MSpec {
    title: "Salve și baionete",
    w: 10,
    h: 8,
    hex: false,
    diag: false,
    map: "........../........../#........./~.~......./.......~.~/.........#/........../..........",
    uts: &[
        UT {
            sym: 'l',
            name: "Linie",
            hp: 3,
            mv: 2,
            rng: 3,
            dmg: 2,
            arm: 0,
            fl: "",
        },
        UT {
            sym: 'g',
            name: "Grenadier",
            hp: 3,
            mv: 3,
            rng: 1,
            dmg: 3,
            arm: 1,
            fl: "C",
        },
        UT {
            sym: 't',
            name: "Tun",
            hp: 2,
            mv: 1,
            rng: 6,
            dmg: 3,
            arm: 0,
            fl: "H",
        },
        UT {
            sym: 'h',
            name: "Husar",
            hp: 3,
            mv: 5,
            rng: 1,
            dmg: 2,
            arm: 0,
            fl: "F",
        },
    ],
    army: ["lllgth", "lllgth"],
    goal: 0,
    act: 4,
    turns: 10,
    target: 18,
    fl: "r",
};
pub static ARENA_PIETRELOR: MSpec = MSpec {
    title: "Arena pietrelor",
    w: 6,
    h: 6,
    hex: true,
    diag: false,
    map: "....../....../~**.../...**~/....../......",
    uts: &[
        UT {
            sym: 'r',
            name: "Ritualist",
            hp: 2,
            mv: 3,
            rng: 3,
            dmg: 2,
            arm: 0,
            fl: "",
        },
        UT {
            sym: 'l',
            name: "Luptător",
            hp: 3,
            mv: 3,
            rng: 1,
            dmg: 3,
            arm: 1,
            fl: "K",
        },
        UT {
            sym: 's',
            name: "Străjer",
            hp: 3,
            mv: 2,
            rng: 1,
            dmg: 2,
            arm: 2,
            fl: "",
        },
    ],
    army: ["rlls", "rlls"],
    goal: 1,
    act: 4,
    turns: 8,
    target: 7,
    fl: "dfp",
};
pub static CHEMAREA_HOARDELOR: MSpec = MSpec {
    title: "Chemarea hoardelor",
    w: 8,
    h: 7,
    hex: false,
    diag: true,
    map: "......../......../.#~#~.../..~..~../...~#~#./......../........",
    uts: &[
        UT {
            sym: 'h',
            name: "Hoardă",
            hp: 2,
            mv: 3,
            rng: 1,
            dmg: 3,
            arm: 0,
            fl: "C",
        },
        UT {
            sym: 's',
            name: "Șaman",
            hp: 2,
            mv: 3,
            rng: 3,
            dmg: 2,
            arm: 0,
            fl: "M",
        },
        UT {
            sym: 'm',
            name: "Monstru",
            hp: 5,
            mv: 3,
            rng: 1,
            dmg: 4,
            arm: 1,
            fl: "K",
        },
        UT {
            sym: 'a',
            name: "Aruncător",
            hp: 1,
            mv: 4,
            rng: 4,
            dmg: 1,
            arm: 0,
            fl: "",
        },
    ],
    army: ["hhhsa", "hhmsa"],
    goal: 0,
    act: 4,
    turns: 9,
    target: 14,
    fl: "dxc",
};
pub static ESCADRE_IN_NEBULOASA: MSpec = MSpec {
    title: "Escadre în nebuloasă",
    w: 10,
    h: 8,
    hex: true,
    diag: false,
    map: "........../........../.....#~#../.~...~..../....~...~./..#~#...../........../..........",
    uts: &[
        UT {
            sym: 'c',
            name: "Crucișător",
            hp: 7,
            mv: 2,
            rng: 4,
            dmg: 4,
            arm: 2,
            fl: "H",
        },
        UT {
            sym: 'f',
            name: "Fregată",
            hp: 4,
            mv: 3,
            rng: 3,
            dmg: 3,
            arm: 1,
            fl: "",
        },
        UT {
            sym: 'i',
            name: "Interceptor",
            hp: 1,
            mv: 6,
            rng: 2,
            dmg: 2,
            arm: 0,
            fl: "L",
        },
        UT {
            sym: 'p',
            name: "Portavion",
            hp: 6,
            mv: 1,
            rng: 3,
            dmg: 2,
            arm: 3,
            fl: "HM",
        },
    ],
    army: ["cff", "cfp"],
    goal: 0,
    act: 3,
    turns: 9,
    target: 0,
    fl: "dr",
};
pub static BLINDATE_PE_SOSEA: MSpec = MSpec {
    title: "Blindate pe șosea",
    w: 9,
    h: 9,
    hex: false,
    diag: true,
    map:
        "........./........./........./#.~~...~./...#.#.../.~...~~.#/........./........./.........",
    uts: &[
        UT {
            sym: 'b',
            name: "Blindat",
            hp: 4,
            mv: 3,
            rng: 4,
            dmg: 3,
            arm: 2,
            fl: "H",
        },
        UT {
            sym: 't',
            name: "Transportor",
            hp: 3,
            mv: 4,
            rng: 1,
            dmg: 1,
            arm: 1,
            fl: "",
        },
        UT {
            sym: 'i',
            name: "Infanterist",
            hp: 2,
            mv: 3,
            rng: 3,
            dmg: 2,
            arm: 0,
            fl: "",
        },
        UT {
            sym: 'a',
            name: "Antitanc",
            hp: 2,
            mv: 2,
            rng: 5,
            dmg: 4,
            arm: 0,
            fl: "H",
        },
    ],
    army: ["bbtiti", "bbtiti"],
    goal: 2,
    act: 3,
    turns: 12,
    target: 6,
    fl: "dr",
};
pub static PATRULE_IN_LAN: MSpec = MSpec {
    title: "Patrule în lan",
    w: 9,
    h: 7,
    hex: false,
    diag: true,
    map: "........./........./~..~...##/.~*.*.*~./##...~..~/........./.........",
    uts: &[
        UT {
            sym: 'f',
            name: "Pușcaș",
            hp: 2,
            mv: 3,
            rng: 4,
            dmg: 2,
            arm: 0,
            fl: "",
        },
        UT {
            sym: 'm',
            name: "Mitralior",
            hp: 2,
            mv: 2,
            rng: 6,
            dmg: 3,
            arm: 0,
            fl: "H",
        },
        UT {
            sym: 'c',
            name: "Comandant",
            hp: 3,
            mv: 3,
            rng: 2,
            dmg: 2,
            arm: 1,
            fl: "F",
        },
        UT {
            sym: 's',
            name: "Sanitar",
            hp: 2,
            mv: 3,
            rng: 1,
            dmg: 1,
            arm: 0,
            fl: "M",
        },
    ],
    army: ["ffmcs", "ffmcs"],
    goal: 1,
    act: 3,
    turns: 9,
    target: 9,
    fl: "drv",
};
pub static VRAJITORI_IN_GER: MSpec = MSpec {
    title: "Vrăjitori în ger",
    w: 8,
    h: 8,
    hex: false,
    diag: true,
    map: "......../......../..~.##../~.~.~~../..~~.~.~/..##.~../......../........",
    uts: &[
        UT {
            sym: 'v',
            name: "Vrăjitor",
            hp: 3,
            mv: 3,
            rng: 5,
            dmg: 3,
            arm: 0,
            fl: "B",
        },
        UT {
            sym: 'a',
            name: "Aprentis",
            hp: 2,
            mv: 3,
            rng: 3,
            dmg: 2,
            arm: 0,
            fl: "",
        },
        UT {
            sym: 's',
            name: "Soldat de gheață",
            hp: 3,
            mv: 3,
            rng: 1,
            dmg: 2,
            arm: 1,
            fl: "",
        },
        UT {
            sym: 'l',
            name: "Lup",
            hp: 2,
            mv: 5,
            rng: 1,
            dmg: 2,
            arm: 0,
            fl: "C",
        },
    ],
    army: ["vasss", "vassl"],
    goal: 0,
    act: 4,
    turns: 10,
    target: 0,
    fl: "dv",
};
pub static BOLIZI_DE_PUSTIU: MSpec = MSpec {
    title: "Bolizi de pustiu",
    w: 10,
    h: 7,
    hex: false,
    diag: true,
    map: "........../........../~........./.~#....#~./.........~/........../..........",
    uts: &[
        UT {
            sym: 'b',
            name: "Buggy",
            hp: 2,
            mv: 6,
            rng: 2,
            dmg: 2,
            arm: 0,
            fl: "L",
        },
        UT {
            sym: 'c',
            name: "Camion",
            hp: 4,
            mv: 4,
            rng: 1,
            dmg: 3,
            arm: 2,
            fl: "CK",
        },
        UT {
            sym: 'm',
            name: "Motocicletă",
            hp: 1,
            mv: 7,
            rng: 1,
            dmg: 2,
            arm: 0,
            fl: "L",
        },
    ],
    army: ["bbcm", "bmmc"],
    goal: 0,
    act: 3,
    turns: 9,
    target: 0,
    fl: "dc",
};
pub static LEGIUNI_SI_COHORTE: MSpec = MSpec {
    title: "Legiuni și cohorte",
    w: 10,
    h: 8,
    hex: false,
    diag: false,
    map: "........../........../#...~...../~........./.........~/.....~...#/........../..........",
    uts: &[
        UT {
            sym: 'l',
            name: "Legionar",
            hp: 4,
            mv: 2,
            rng: 1,
            dmg: 3,
            arm: 2,
            fl: "",
        },
        UT {
            sym: 'a',
            name: "Arcaș",
            hp: 2,
            mv: 3,
            rng: 4,
            dmg: 2,
            arm: 0,
            fl: "",
        },
        UT {
            sym: 'c',
            name: "Călăreț",
            hp: 3,
            mv: 5,
            rng: 1,
            dmg: 3,
            arm: 0,
            fl: "C",
        },
        UT {
            sym: 't',
            name: "Triarii",
            hp: 5,
            mv: 2,
            rng: 1,
            dmg: 3,
            arm: 3,
            fl: "H",
        },
    ],
    army: ["lllact", "lllcca"],
    goal: 0,
    act: 4,
    turns: 11,
    target: 20,
    fl: "f",
};
pub static FALANGE_SI_CALARIME: MSpec = MSpec {
    title: "Falangă și călărime",
    w: 10,
    h: 7,
    hex: false,
    diag: false,
    map: "........../........../....#~..../~........~/....~#..../........../..........",
    uts: &[
        UT {
            sym: 'f',
            name: "Falangist",
            hp: 4,
            mv: 2,
            rng: 1,
            dmg: 3,
            arm: 3,
            fl: "H",
        },
        UT {
            sym: 'c',
            name: "Cavalerist",
            hp: 3,
            mv: 5,
            rng: 1,
            dmg: 3,
            arm: 1,
            fl: "C",
        },
        UT {
            sym: 'p',
            name: "Praștiaș",
            hp: 1,
            mv: 3,
            rng: 3,
            dmg: 1,
            arm: 0,
            fl: "",
        },
        UT {
            sym: 'e',
            name: "Elefant",
            hp: 6,
            mv: 3,
            rng: 1,
            dmg: 4,
            arm: 2,
            fl: "KC",
        },
    ],
    army: ["fffcp", "ffcce"],
    goal: 0,
    act: 4,
    turns: 11,
    target: 20,
    fl: "cf",
};
pub static GARDIENI_SI_CORSARI: MSpec = MSpec {
    title: "Gardieni și corsari",
    w: 8,
    h: 8,
    hex: false,
    diag: true,
    map: "......../......../#...~.~./..**..../....**../.~.~...#/......../........",
    uts: &[
        UT {
            sym: 'g',
            name: "Gardian",
            hp: 3,
            mv: 3,
            rng: 1,
            dmg: 3,
            arm: 1,
            fl: "F",
        },
        UT {
            sym: 'k',
            name: "Corsar",
            hp: 2,
            mv: 4,
            rng: 3,
            dmg: 2,
            arm: 0,
            fl: "",
        },
        UT {
            sym: 'm',
            name: "Mentor",
            hp: 3,
            mv: 3,
            rng: 2,
            dmg: 3,
            arm: 1,
            fl: "B",
        },
        UT {
            sym: 'd',
            name: "Droid",
            hp: 2,
            mv: 3,
            rng: 4,
            dmg: 2,
            arm: 1,
            fl: "",
        },
    ],
    army: ["gkmd", "gkkd"],
    goal: 1,
    act: 3,
    turns: 9,
    target: 8,
    fl: "dxp",
};
pub static CAROSATE_DE_FIER: MSpec = MSpec {
    title: "Carosate de fier",
    w: 10,
    h: 10,
    hex: false,
    diag: true,
    map: "........../........../...##..~../..~...~.../~...#.~.../...~.#...~/...~...~../..~..##.../........../..........",
    uts: &[
        UT { sym: 't', name: "Tanc", hp: 4, mv: 3, rng: 4, dmg: 3, arm: 2, fl: "H" },
        UT { sym: 'i', name: "Infanterie", hp: 2, mv: 2, rng: 3, dmg: 2, arm: 1, fl: "" },
        UT { sym: 'm', name: "Mersător", hp: 6, mv: 2, rng: 5, dmg: 4, arm: 2, fl: "HB" },
        UT { sym: 'a', name: "Aviator", hp: 2, mv: 7, rng: 2, dmg: 2, arm: 0, fl: "L" },
    ],
    army: ["ttiimm", "ttiita"],
    goal: 0,
    act: 4,
    turns: 11,
    target: 24,
    fl: "r",
};
pub static TITANI_DE_CEATA: MSpec = MSpec {
    title: "Titani de ceață",
    w: 9,
    h: 9,
    hex: true,
    diag: false,
    map:
        "........./........./......~.~/..#.#.#~~/..~...~../~~#.#.#../~.~....../........./.........",
    uts: &[
        UT {
            sym: 't',
            name: "Titan",
            hp: 9,
            mv: 2,
            rng: 5,
            dmg: 5,
            arm: 3,
            fl: "HBK",
        },
        UT {
            sym: 's',
            name: "Scutier",
            hp: 3,
            mv: 3,
            rng: 2,
            dmg: 2,
            arm: 1,
            fl: "M",
        },
        UT {
            sym: 'e',
            name: "Explorator",
            hp: 3,
            mv: 5,
            rng: 3,
            dmg: 3,
            arm: 1,
            fl: "L",
        },
    ],
    army: ["tse", "tss"],
    goal: 0,
    act: 3,
    turns: 9,
    target: 0,
    fl: "drx",
};
pub static NAVE_IN_DERIVA: MSpec = MSpec {
    title: "Nave în derivă",
    w: 10,
    h: 9,
    hex: true,
    diag: false,
    map: "........../........../.#....~.../..~..#..../.~..~~..~./....#..~../...~....#./........../..........",
    uts: &[
        UT { sym: 'c', name: "Cuirasat", hp: 8, mv: 1, rng: 5, dmg: 4, arm: 3, fl: "H" },
        UT { sym: 'f', name: "Fregată", hp: 4, mv: 3, rng: 3, dmg: 3, arm: 1, fl: "" },
        UT { sym: 'e', name: "Escortă", hp: 2, mv: 5, rng: 2, dmg: 2, arm: 0, fl: "L" },
    ],
    army: ["cfee", "cfff"],
    goal: 0,
    act: 3,
    turns: 10,
    target: 0,
    fl: "dr",
};
pub static BALADA_VITEJILOR: MSpec = MSpec {
    title: "Balada vitejilor",
    w: 7,
    h: 7,
    hex: false,
    diag: false,
    map: "......./......./......~/~#~.~#~/~....../......./.......",
    uts: &[
        UT {
            sym: 'e',
            name: "Erou",
            hp: 4,
            mv: 3,
            rng: 1,
            dmg: 4,
            arm: 1,
            fl: "F",
        },
        UT {
            sym: 't',
            name: "Tâlhar",
            hp: 2,
            mv: 4,
            rng: 1,
            dmg: 2,
            arm: 0,
            fl: "",
        },
        UT {
            sym: 'a',
            name: "Arcaș",
            hp: 1,
            mv: 3,
            rng: 4,
            dmg: 2,
            arm: 0,
            fl: "",
        },
        UT {
            sym: 'm',
            name: "Menestrel",
            hp: 2,
            mv: 3,
            rng: 1,
            dmg: 1,
            arm: 0,
            fl: "M",
        },
    ],
    army: ["eta", "etam"],
    goal: 0,
    act: 2,
    turns: 12,
    target: 0,
    fl: "dxf",
};
pub static ECHIPAJ_DE_EPAVA: MSpec = MSpec {
    title: "Echipaj de epavă",
    w: 8,
    h: 8,
    hex: false,
    diag: true,
    map: "......../......../#~.~..#./.~**...~/~...**~./.#..~.~#/......../........",
    uts: &[
        UT {
            sym: 'k',
            name: "Căpitan",
            hp: 3,
            mv: 3,
            rng: 3,
            dmg: 3,
            arm: 1,
            fl: "",
        },
        UT {
            sym: 's',
            name: "Salvator",
            hp: 2,
            mv: 4,
            rng: 1,
            dmg: 2,
            arm: 0,
            fl: "",
        },
        UT {
            sym: 'r',
            name: "Robot",
            hp: 3,
            mv: 2,
            rng: 2,
            dmg: 2,
            arm: 2,
            fl: "H",
        },
        UT {
            sym: 'p',
            name: "Pilot",
            hp: 2,
            mv: 5,
            rng: 3,
            dmg: 2,
            arm: 0,
            fl: "L",
        },
    ],
    army: ["ksrp", "ksrp"],
    goal: 1,
    act: 3,
    turns: 8,
    target: 7,
    fl: "dv",
};
pub static ARMATE_DE_BUZUNAR: MSpec = MSpec {
    title: "Armate de buzunar",
    w: 11,
    h: 9,
    hex: false,
    diag: true,
    map: ".........../.........../.~.~.~~..#./........#../..#.....#../..#......../.#..~~.~.~./.........../...........",
    uts: &[
        UT { sym: 'i', name: "Infanterie", hp: 2, mv: 2, rng: 2, dmg: 2, arm: 1, fl: "" },
        UT { sym: 't', name: "Tanc", hp: 4, mv: 4, rng: 3, dmg: 3, arm: 2, fl: "" },
        UT { sym: 'a', name: "Artilerie", hp: 2, mv: 1, rng: 6, dmg: 4, arm: 0, fl: "H" },
        UT { sym: 'z', name: "Zburător", hp: 3, mv: 7, rng: 2, dmg: 2, arm: 0, fl: "L" },
    ],
    army: ["iiiita", "iiiitaz"],
    goal: 0,
    act: 5,
    turns: 12,
    target: 22,
    fl: "",
};
pub static ROATA_PUTERILOR: MSpec = MSpec {
    title: "Roata puterilor",
    w: 8,
    h: 8,
    hex: false,
    diag: true,
    map: "......../......../~......./.#.~...~/~...~.#./.......~/......../........",
    uts: &[
        UT {
            sym: 'v',
            name: "Viteaz",
            hp: 3,
            mv: 4,
            rng: 1,
            dmg: 3,
            arm: 1,
            fl: "",
        },
        UT {
            sym: 'g',
            name: "Gigant",
            hp: 5,
            mv: 2,
            rng: 1,
            dmg: 4,
            arm: 2,
            fl: "K",
        },
        UT {
            sym: 'm',
            name: "Mag",
            hp: 2,
            mv: 3,
            rng: 4,
            dmg: 3,
            arm: 0,
            fl: "B",
        },
        UT {
            sym: 'z',
            name: "Zburător",
            hp: 2,
            mv: 5,
            rng: 2,
            dmg: 2,
            arm: 0,
            fl: "L",
        },
    ],
    army: ["vgmz", "vgmz"],
    goal: 0,
    act: 3,
    turns: 10,
    target: 0,
    fl: "dpx",
};
pub static ESCADRILE_DE_HARTIE: MSpec = MSpec {
    title: "Escadrile de hârtie",
    w: 10,
    h: 10,
    hex: false,
    diag: true,
    map: "........../........../.......~../.~.#....../...~.~..../....~.~.../......#.~./..~......./........../..........",
    uts: &[
        UT { sym: 'v', name: "Vânător", hp: 2, mv: 6, rng: 2, dmg: 2, arm: 0, fl: "L" },
        UT { sym: 'b', name: "Bombardier", hp: 3, mv: 4, rng: 2, dmg: 3, arm: 1, fl: "L" },
        UT { sym: 'a', name: "As", hp: 2, mv: 7, rng: 3, dmg: 3, arm: 0, fl: "L" },
    ],
    army: ["vvba", "vvva"],
    goal: 0,
    act: 6,
    turns: 10,
    target: 0,
    fl: "d",
};
pub static MERSATORI_SI_PUSASI: MSpec = MSpec {
    title: "Mersători și pușcași",
    w: 8,
    h: 8,
    hex: false,
    diag: true,
    map: "......../......../..~#~.../.~..~.#./.#.~..~./...~#~../......../........",
    uts: &[
        UT {
            sym: 'm',
            name: "Mersător",
            hp: 5,
            mv: 3,
            rng: 3,
            dmg: 4,
            arm: 2,
            fl: "H",
        },
        UT {
            sym: 'p',
            name: "Pușcaș",
            hp: 2,
            mv: 3,
            rng: 4,
            dmg: 2,
            arm: 0,
            fl: "",
        },
        UT {
            sym: 'g',
            name: "Grenadier",
            hp: 2,
            mv: 3,
            rng: 3,
            dmg: 3,
            arm: 1,
            fl: "B",
        },
        UT {
            sym: 's',
            name: "Scout",
            hp: 1,
            mv: 5,
            rng: 2,
            dmg: 1,
            arm: 0,
            fl: "L",
        },
    ],
    army: ["mppgs", "mppgs"],
    goal: 0,
    act: 4,
    turns: 10,
    target: 0,
    fl: "dr",
};
pub static ATERIZARE_PESTE_ORAS: MSpec = MSpec {
    title: "Aterizare peste oraș",
    w: 9,
    h: 9,
    hex: false,
    diag: true,
    map:
        "........./........./~.#.~..../~.~.~.#../#.~#.#~.#/..#.~.~.~/....~.#.~/........./.........",
    uts: &[
        UT {
            sym: 'a',
            name: "Aeronavă",
            hp: 3,
            mv: 6,
            rng: 1,
            dmg: 1,
            arm: 1,
            fl: "L",
        },
        UT {
            sym: 't',
            name: "Tanc",
            hp: 4,
            mv: 3,
            rng: 4,
            dmg: 3,
            arm: 2,
            fl: "H",
        },
        UT {
            sym: 's',
            name: "Soldat",
            hp: 2,
            mv: 3,
            rng: 3,
            dmg: 2,
            arm: 0,
            fl: "",
        },
        UT {
            sym: 'w',
            name: "Walker",
            hp: 4,
            mv: 2,
            rng: 3,
            dmg: 3,
            arm: 2,
            fl: "K",
        },
    ],
    army: ["aassw", "aasst"],
    goal: 2,
    act: 4,
    turns: 12,
    target: 8,
    fl: "dr",
};
pub static CASE_SI_IERNI: MSpec = MSpec {
    title: "Case și ierni",
    w: 8,
    h: 8,
    hex: false,
    diag: true,
    map: "......../......../...~..../~.**.#../..#.**.~/....~.../......../........",
    uts: &[
        UT {
            sym: 'c',
            name: "Cavaler",
            hp: 4,
            mv: 4,
            rng: 1,
            dmg: 3,
            arm: 2,
            fl: "C",
        },
        UT {
            sym: 'l',
            name: "Lăncier",
            hp: 2,
            mv: 3,
            rng: 1,
            dmg: 2,
            arm: 1,
            fl: "F",
        },
        UT {
            sym: 'a',
            name: "Arcaș",
            hp: 1,
            mv: 3,
            rng: 4,
            dmg: 2,
            arm: 0,
            fl: "",
        },
        UT {
            sym: 'b',
            name: "Bastard",
            hp: 3,
            mv: 3,
            rng: 1,
            dmg: 3,
            arm: 1,
            fl: "",
        },
    ],
    army: ["cllaa", "bllla"],
    goal: 1,
    act: 3,
    turns: 9,
    target: 10,
    fl: "dcf",
};
pub static OSTI_DE_HOTAR: MSpec = MSpec {
    title: "Oști de hotar",
    w: 10,
    h: 8,
    hex: false,
    diag: false,
    map: "........../........../....~...../......#~../..~#....../.....~..../........../..........",
    uts: &[
        UT {
            sym: 'z',
            name: "Zid de scuturi",
            hp: 4,
            mv: 2,
            rng: 1,
            dmg: 3,
            arm: 3,
            fl: "H",
        },
        UT {
            sym: 't',
            name: "Tăietor",
            hp: 3,
            mv: 3,
            rng: 1,
            dmg: 4,
            arm: 1,
            fl: "",
        },
        UT {
            sym: 'a',
            name: "Arbaletier",
            hp: 2,
            mv: 3,
            rng: 4,
            dmg: 3,
            arm: 0,
            fl: "",
        },
        UT {
            sym: 'c',
            name: "Cavaler",
            hp: 4,
            mv: 5,
            rng: 1,
            dmg: 3,
            arm: 2,
            fl: "C",
        },
    ],
    army: ["zzzta", "ztttca"],
    goal: 0,
    act: 4,
    turns: 11,
    target: 22,
    fl: "cf",
};
pub static PUSASI_PE_PLANETA_ROSIE: MSpec = MSpec {
    title: "Pușcași pe planeta roșie",
    w: 9,
    h: 8,
    hex: false,
    diag: true,
    map: "........./........./#.....~#./.~.~~..../....~~.~./.#~.....#/........./.........",
    uts: &[
        UT {
            sym: 'p',
            name: "Pușcaș",
            hp: 2,
            mv: 3,
            rng: 5,
            dmg: 2,
            arm: 0,
            fl: "",
        },
        UT {
            sym: 'e',
            name: "Exo-armură",
            hp: 5,
            mv: 2,
            rng: 3,
            dmg: 4,
            arm: 2,
            fl: "H",
        },
        UT {
            sym: 's',
            name: "Săritor",
            hp: 2,
            mv: 5,
            rng: 1,
            dmg: 3,
            arm: 0,
            fl: "LC",
        },
        UT {
            sym: 'm',
            name: "Medic",
            hp: 2,
            mv: 3,
            rng: 1,
            dmg: 1,
            arm: 0,
            fl: "M",
        },
    ],
    army: ["pppesm", "pppesm"],
    goal: 0,
    act: 4,
    turns: 10,
    target: 0,
    fl: "dvx",
};
pub static CORIDORUL_RUINAT: MSpec = MSpec {
    title: "Coridorul ruinat",
    w: 6,
    h: 6,
    hex: false,
    diag: false,
    map: "....../....../#~#~~#/#~~#~#/....../......",
    uts: &[
        UT {
            sym: 's',
            name: "Soldat",
            hp: 2,
            mv: 3,
            rng: 4,
            dmg: 2,
            arm: 1,
            fl: "",
        },
        UT {
            sym: 't',
            name: "Tunar",
            hp: 2,
            mv: 2,
            rng: 5,
            dmg: 3,
            arm: 0,
            fl: "H",
        },
        UT {
            sym: 'm',
            name: "Mercenar",
            hp: 3,
            mv: 3,
            rng: 1,
            dmg: 3,
            arm: 1,
            fl: "C",
        },
    ],
    army: ["sstm", "sstm"],
    goal: 0,
    act: 3,
    turns: 8,
    target: 0,
    fl: "dr",
};
pub static RINGUL_CU_BILA: MSpec = MSpec {
    title: "Ringul cu bilă",
    w: 7,
    h: 9,
    hex: false,
    diag: false,
    map: "......./......./......./~....../......./......~/......./......./.......",
    uts: &[
        UT {
            sym: 'a',
            name: "Atacant",
            hp: 2,
            mv: 4,
            rng: 1,
            dmg: 2,
            arm: 0,
            fl: "",
        },
        UT {
            sym: 't',
            name: "Tank",
            hp: 4,
            mv: 2,
            rng: 1,
            dmg: 3,
            arm: 2,
            fl: "KH",
        },
        UT {
            sym: 'r',
            name: "Rapid",
            hp: 1,
            mv: 5,
            rng: 1,
            dmg: 1,
            arm: 0,
            fl: "L",
        },
    ],
    army: ["aaatr", "aaatr"],
    goal: 2,
    act: 4,
    turns: 14,
    target: 6,
    fl: "pf",
};
pub static DOMNI_SI_ARCASI: MSpec = MSpec {
    title: "Domni și arcași",
    w: 9,
    h: 8,
    hex: false,
    diag: false,
    map: "........./........./........#/..*.*.~.~/~.~.*.*../#......../........./.........",
    uts: &[
        UT {
            sym: 'c',
            name: "Călăreț",
            hp: 3,
            mv: 5,
            rng: 1,
            dmg: 3,
            arm: 1,
            fl: "C",
        },
        UT {
            sym: 's',
            name: "Sergent",
            hp: 2,
            mv: 3,
            rng: 1,
            dmg: 2,
            arm: 1,
            fl: "F",
        },
        UT {
            sym: 'a',
            name: "Arcaș",
            hp: 1,
            mv: 3,
            rng: 4,
            dmg: 2,
            arm: 0,
            fl: "",
        },
        UT {
            sym: 'd',
            name: "Domn",
            hp: 4,
            mv: 3,
            rng: 1,
            dmg: 4,
            arm: 2,
            fl: "",
        },
    ],
    army: ["dssaa", "dscaa"],
    goal: 1,
    act: 3,
    turns: 10,
    target: 9,
    fl: "dcf",
};
pub static BASM_CU_BALAURI: MSpec = MSpec {
    title: "Basm cu balauri",
    w: 8,
    h: 8,
    hex: false,
    diag: true,
    map: "......../......../.....~.#/..~.~.../...~.~../#.~...../......../........",
    uts: &[
        UT {
            sym: 'b',
            name: "Balaur",
            hp: 6,
            mv: 5,
            rng: 2,
            dmg: 4,
            arm: 2,
            fl: "LB",
        },
        UT {
            sym: 'c',
            name: "Cavaler",
            hp: 3,
            mv: 3,
            rng: 1,
            dmg: 3,
            arm: 2,
            fl: "C",
        },
        UT {
            sym: 'v',
            name: "Vrăjitoare",
            hp: 2,
            mv: 3,
            rng: 4,
            dmg: 2,
            arm: 0,
            fl: "M",
        },
        UT {
            sym: 'g',
            name: "Goblin",
            hp: 1,
            mv: 4,
            rng: 1,
            dmg: 2,
            arm: 0,
            fl: "",
        },
    ],
    army: ["bcv", "cgggg"],
    goal: 0,
    act: 3,
    turns: 10,
    target: 0,
    fl: "dxc",
};
pub static AVIOANE_IN_AMURG: MSpec = MSpec {
    title: "Avioane în amurg",
    w: 10,
    h: 10,
    hex: false,
    diag: true,
    map: "........../........../..~......./.#.....~.~/.....~..../....~...../~.~.....#./.......~../........../..........",
    uts: &[
        UT { sym: 'a', name: "Vânător", hp: 2, mv: 6, rng: 2, dmg: 2, arm: 0, fl: "L" },
        UT { sym: 'b', name: "Bombardier", hp: 4, mv: 4, rng: 2, dmg: 3, arm: 1, fl: "L" },
        UT { sym: 'e', name: "As", hp: 3, mv: 7, rng: 3, dmg: 3, arm: 1, fl: "L" },
    ],
    army: ["aaab", "aaee"],
    goal: 0,
    act: 5,
    turns: 10,
    target: 0,
    fl: "d",
};
pub static TORPILOARE_IN_NOAPTE: MSpec = MSpec {
    title: "Torpiloare în noapte",
    w: 10,
    h: 8,
    hex: true,
    diag: false,
    map: "........../........../...~.#.~~./.....~...#/#...~...../.~~.#.~.../........../..........",
    uts: &[
        UT {
            sym: 't',
            name: "Torpilor",
            hp: 2,
            mv: 5,
            rng: 3,
            dmg: 3,
            arm: 0,
            fl: "L",
        },
        UT {
            sym: 'e',
            name: "Escortor",
            hp: 4,
            mv: 3,
            rng: 3,
            dmg: 3,
            arm: 1,
            fl: "",
        },
        UT {
            sym: 'c',
            name: "Cargou",
            hp: 6,
            mv: 2,
            rng: 1,
            dmg: 1,
            arm: 2,
            fl: "H",
        },
    ],
    army: ["ttc", "tee"],
    goal: 0,
    act: 4,
    turns: 9,
    target: 0,
    fl: "dr",
};
pub static CORABII_CU_TUNURI: MSpec = MSpec {
    title: "Corăbii cu tunuri",
    w: 10,
    h: 9,
    hex: true,
    diag: false,
    map: "........../........../...~.~~.#./#~....#~../........../..~#....~#/.#.~~.~.../........../..........",
    uts: &[
        UT { sym: 'f', name: "Fregată", hp: 5, mv: 3, rng: 3, dmg: 3, arm: 1, fl: "" },
        UT { sym: 'g', name: "Galion", hp: 7, mv: 2, rng: 3, dmg: 4, arm: 2, fl: "H" },
        UT { sym: 's', name: "Șalupă", hp: 2, mv: 5, rng: 2, dmg: 2, arm: 0, fl: "L" },
    ],
    army: ["ffg", "fgs"],
    goal: 0,
    act: 3,
    turns: 11,
    target: 0,
    fl: "dx",
};
pub static PANZE_SI_GHIULELE: MSpec = MSpec {
    title: "Pânze și ghiulele",
    w: 9,
    h: 9,
    hex: true,
    diag: false,
    map:
        "........./........./..~~...../.~#....~./...#.#.../.~....#~./.....~~../........./.........",
    uts: &[
        UT {
            sym: 'b',
            name: "Brig",
            hp: 4,
            mv: 4,
            rng: 3,
            dmg: 3,
            arm: 1,
            fl: "",
        },
        UT {
            sym: 'f',
            name: "Fregată",
            hp: 5,
            mv: 3,
            rng: 3,
            dmg: 3,
            arm: 2,
            fl: "",
        },
        UT {
            sym: 'c',
            name: "Cutter",
            hp: 2,
            mv: 5,
            rng: 2,
            dmg: 2,
            arm: 0,
            fl: "L",
        },
    ],
    army: ["bbf", "bff"],
    goal: 0,
    act: 3,
    turns: 10,
    target: 0,
    fl: "dv",
};
pub static PE_VALURI_DE_OTEL: MSpec = MSpec {
    title: "Pe valuri de oțel",
    w: 11,
    h: 8,
    hex: false,
    diag: true,
    map: ".........../.........../~.........#/.....~.#.~./.~.#.~...../#.........~/.........../...........",
    uts: &[
        UT { sym: 'c', name: "Cuirasat", hp: 8, mv: 2, rng: 6, dmg: 5, arm: 3, fl: "H" },
        UT { sym: 'd', name: "Distrugător", hp: 3, mv: 5, rng: 3, dmg: 3, arm: 1, fl: "L" },
        UT { sym: 'k', name: "Crucișător", hp: 5, mv: 3, rng: 4, dmg: 3, arm: 2, fl: "" },
    ],
    army: ["cdd", "kkd"],
    goal: 0,
    act: 3,
    turns: 10,
    target: 0,
    fl: "dr",
};
pub static FLOTE_CU_ABURI: MSpec = MSpec {
    title: "Flote cu aburi",
    w: 10,
    h: 8,
    hex: true,
    diag: false,
    map: "........../........../~~~.~..#../~..##...../.....##..~/..#..~.~~~/........../..........",
    uts: &[
        UT {
            sym: 'n',
            name: "Monitor",
            hp: 6,
            mv: 2,
            rng: 4,
            dmg: 4,
            arm: 3,
            fl: "HB",
        },
        UT {
            sym: 'v',
            name: "Vapor rapid",
            hp: 3,
            mv: 4,
            rng: 3,
            dmg: 3,
            arm: 1,
            fl: "",
        },
        UT {
            sym: 'z',
            name: "Zeppelin",
            hp: 3,
            mv: 3,
            rng: 2,
            dmg: 3,
            arm: 1,
            fl: "L",
        },
        UT {
            sym: 't',
            name: "Tender",
            hp: 3,
            mv: 3,
            rng: 1,
            dmg: 1,
            arm: 1,
            fl: "M",
        },
    ],
    army: ["nvzt", "nnvt"],
    goal: 0,
    act: 3,
    turns: 10,
    target: 0,
    fl: "drx",
};
pub static FLOTE_IN_INEL: MSpec = MSpec {
    title: "Flote în inel",
    w: 9,
    h: 9,
    hex: true,
    diag: false,
    map:
        "........./........./.......#./.~~....../..*.*.*../......~~./.#......./........./.........",
    uts: &[
        UT {
            sym: 'c',
            name: "Cruiser",
            hp: 5,
            mv: 3,
            rng: 4,
            dmg: 3,
            arm: 2,
            fl: "",
        },
        UT {
            sym: 'e',
            name: "Escortă",
            hp: 2,
            mv: 5,
            rng: 2,
            dmg: 2,
            arm: 0,
            fl: "L",
        },
        UT {
            sym: 'd',
            name: "Dronă",
            hp: 1,
            mv: 6,
            rng: 1,
            dmg: 2,
            arm: 0,
            fl: "L",
        },
    ],
    army: ["cced", "ceee"],
    goal: 1,
    act: 4,
    turns: 9,
    target: 9,
    fl: "drp",
};
pub static FREGATE_IN_VID: MSpec = MSpec {
    title: "Fregate în vid",
    w: 12,
    h: 9,
    hex: false,
    diag: true,
    map: "............/............/~..~..#...../............/............/............/.....#..~..~/............/............",
    uts: &[
        UT { sym: 'f', name: "Fregată", hp: 4, mv: 3, rng: 4, dmg: 3, arm: 1, fl: "" },
        UT { sym: 'c', name: "Cuirasat", hp: 7, mv: 2, rng: 5, dmg: 4, arm: 2, fl: "H" },
        UT { sym: 'r', name: "Rapid", hp: 2, mv: 5, rng: 2, dmg: 2, arm: 0, fl: "L" },
    ],
    army: ["ffcr", "ffcr"],
    goal: 0,
    act: 3,
    turns: 12,
    target: 0,
    fl: "",
};
pub static SCUTURI_SI_RAZE: MSpec = MSpec {
    title: "Scuturi și raze",
    w: 10,
    h: 8,
    hex: true,
    diag: false,
    map: "........../........../...#.#.~../~.~......~/~......~.~/..~.#.#.../........../..........",
    uts: &[
        UT {
            sym: 's',
            name: "Scutier",
            hp: 5,
            mv: 3,
            rng: 3,
            dmg: 4,
            arm: 3,
            fl: "",
        },
        UT {
            sym: 'r',
            name: "Rapid",
            hp: 3,
            mv: 5,
            rng: 3,
            dmg: 3,
            arm: 1,
            fl: "L",
        },
        UT {
            sym: 'm',
            name: "Medical",
            hp: 4,
            mv: 3,
            rng: 1,
            dmg: 1,
            arm: 1,
            fl: "M",
        },
    ],
    army: ["ssrm", "ssrr"],
    goal: 0,
    act: 3,
    turns: 11,
    target: 0,
    fl: "",
};
pub static ORBITA_IN_FLACARI: MSpec = MSpec {
    title: "Orbită în flăcări",
    w: 9,
    h: 8,
    hex: true,
    diag: false,
    map: "........./........./.......~./..*.*.~#./.#~.*.*../.~......./........./.........",
    uts: &[
        UT {
            sym: 'f',
            name: "Fregată",
            hp: 4,
            mv: 3,
            rng: 4,
            dmg: 3,
            arm: 1,
            fl: "",
        },
        UT {
            sym: 'l',
            name: "Lansator",
            hp: 3,
            mv: 3,
            rng: 2,
            dmg: 2,
            arm: 1,
            fl: "L",
        },
        UT {
            sym: 'g',
            name: "Gigant orbital",
            hp: 8,
            mv: 1,
            rng: 5,
            dmg: 5,
            arm: 3,
            fl: "HB",
        },
    ],
    army: ["ffl", "fgl"],
    goal: 1,
    act: 4,
    turns: 9,
    target: 9,
    fl: "dr",
};
pub static NAVE_DE_EXPEDITIE: MSpec = MSpec {
    title: "Nave de expediție",
    w: 9,
    h: 8,
    hex: true,
    diag: false,
    map: "........./........./~.~.~..~./..#...#../..#...#../.~..~.~.~/........./.........",
    uts: &[
        UT {
            sym: 'e',
            name: "Explorator",
            hp: 4,
            mv: 4,
            rng: 3,
            dmg: 3,
            arm: 1,
            fl: "",
        },
        UT {
            sym: 's',
            name: "Scut mobil",
            hp: 3,
            mv: 3,
            rng: 2,
            dmg: 2,
            arm: 2,
            fl: "M",
        },
        UT {
            sym: 'k',
            name: "Vânător",
            hp: 2,
            mv: 6,
            rng: 2,
            dmg: 3,
            arm: 0,
            fl: "L",
        },
    ],
    army: ["ees", "ekk"],
    goal: 0,
    act: 3,
    turns: 10,
    target: 0,
    fl: "dv",
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn eroi_pe_acoperis() {
        check_spec(&EROI_PE_ACOPERIS);
    }
    #[test]
    fn razboinici_de_fiord() {
        check_spec(&RAZBOINICI_DE_FIORD);
    }
    #[test]
    fn osti_pe_o_fila() {
        check_spec(&OSTI_PE_O_FILA);
    }
    #[test]
    fn bande_in_ruine() {
        check_spec(&BANDE_IN_RUINE);
    }
    #[test]
    fn masinarii_de_abur() {
        check_spec(&MASINARII_DE_ABUR);
    }
    #[test]
    fn strigoi_de_targ() {
        check_spec(&STRIGOI_DE_TARG);
    }
    #[test]
    fn giganti_de_otel() {
        check_spec(&GIGANTI_DE_OTEL);
    }
    #[test]
    fn salve_si_baionete() {
        check_spec(&SALVE_SI_BAIONETE);
    }
    #[test]
    fn arena_pietrelor() {
        check_spec(&ARENA_PIETRELOR);
    }
    #[test]
    fn chemarea_hoardelor() {
        check_spec(&CHEMAREA_HOARDELOR);
    }
    #[test]
    fn escadre_in_nebuloasa() {
        check_spec(&ESCADRE_IN_NEBULOASA);
    }
    #[test]
    fn blindate_pe_sosea() {
        check_spec(&BLINDATE_PE_SOSEA);
    }
    #[test]
    fn patrule_in_lan() {
        check_spec(&PATRULE_IN_LAN);
    }
    #[test]
    fn vrajitori_in_ger() {
        check_spec(&VRAJITORI_IN_GER);
    }
    #[test]
    fn bolizi_de_pustiu() {
        check_spec(&BOLIZI_DE_PUSTIU);
    }
    #[test]
    fn legiuni_si_cohorte() {
        check_spec(&LEGIUNI_SI_COHORTE);
    }
    #[test]
    fn falange_si_calarime() {
        check_spec(&FALANGE_SI_CALARIME);
    }
    #[test]
    fn gardieni_si_corsari() {
        check_spec(&GARDIENI_SI_CORSARI);
    }
    #[test]
    fn carosate_de_fier() {
        check_spec(&CAROSATE_DE_FIER);
    }
    #[test]
    fn titani_de_ceata() {
        check_spec(&TITANI_DE_CEATA);
    }
    #[test]
    fn nave_in_deriva() {
        check_spec(&NAVE_IN_DERIVA);
    }
    #[test]
    fn balada_vitejilor() {
        check_spec(&BALADA_VITEJILOR);
    }
    #[test]
    fn echipaj_de_epava() {
        check_spec(&ECHIPAJ_DE_EPAVA);
    }
    #[test]
    fn armate_de_buzunar() {
        check_spec(&ARMATE_DE_BUZUNAR);
    }
    #[test]
    fn roata_puterilor() {
        check_spec(&ROATA_PUTERILOR);
    }
    #[test]
    fn escadrile_de_hartie() {
        check_spec(&ESCADRILE_DE_HARTIE);
    }
    #[test]
    fn mersatori_si_pusasi() {
        check_spec(&MERSATORI_SI_PUSASI);
    }
    #[test]
    fn aterizare_peste_oras() {
        check_spec(&ATERIZARE_PESTE_ORAS);
    }
    #[test]
    fn case_si_ierni() {
        check_spec(&CASE_SI_IERNI);
    }
    #[test]
    fn osti_de_hotar() {
        check_spec(&OSTI_DE_HOTAR);
    }
    #[test]
    fn pusasi_pe_planeta_rosie() {
        check_spec(&PUSASI_PE_PLANETA_ROSIE);
    }
    #[test]
    fn coridorul_ruinat() {
        check_spec(&CORIDORUL_RUINAT);
    }
    #[test]
    fn ringul_cu_bila() {
        check_spec(&RINGUL_CU_BILA);
    }
    #[test]
    fn domni_si_arcasi() {
        check_spec(&DOMNI_SI_ARCASI);
    }
    #[test]
    fn basm_cu_balauri() {
        check_spec(&BASM_CU_BALAURI);
    }
    #[test]
    fn avioane_in_amurg() {
        check_spec(&AVIOANE_IN_AMURG);
    }
    #[test]
    fn torpiloare_in_noapte() {
        check_spec(&TORPILOARE_IN_NOAPTE);
    }
    #[test]
    fn corabii_cu_tunuri() {
        check_spec(&CORABII_CU_TUNURI);
    }
    #[test]
    fn panze_si_ghiulele() {
        check_spec(&PANZE_SI_GHIULELE);
    }
    #[test]
    fn pe_valuri_de_otel() {
        check_spec(&PE_VALURI_DE_OTEL);
    }
    #[test]
    fn flote_cu_aburi() {
        check_spec(&FLOTE_CU_ABURI);
    }
    #[test]
    fn flote_in_inel() {
        check_spec(&FLOTE_IN_INEL);
    }
    #[test]
    fn fregate_in_vid() {
        check_spec(&FREGATE_IN_VID);
    }
    #[test]
    fn scuturi_si_raze() {
        check_spec(&SCUTURI_SI_RAZE);
    }
    #[test]
    fn orbita_in_flacari() {
        check_spec(&ORBITA_IN_FLACARI);
    }
    #[test]
    fn nave_de_expeditie() {
        check_spec(&NAVE_DE_EXPEDITIE);
    }
}
