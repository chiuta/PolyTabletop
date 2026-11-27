//! Date pentru motorul `t4_tcg` (generat de t4gen/gen.py): cărți și pachete fixe pe joc.
use crate::t4_tcg::*;

macro_rules! c {
    ($n:expr, $co:expr, $p:expr, $h:expr, $k:expr, $fx:expr, $civ:expr, $l:expr, $f:expr) => {
        Card {
            name: $n,
            cost: $co,
            pow: $p,
            hp: $h,
            kind: $k,
            fx: $fx,
            civ: $civ,
            lvl: $l,
            fl: $f,
        }
    };
}

const C_PIRATI_DE_HARTIE: &[Card] = &[
    c!("Mus de bord", 1, 2, 1, 0, 0, 0, 0, 0),
    c!("Marinar", 2, 3, 2, 0, 0, 0, 0, 0),
    c!("Tunar cu scut", 2, 2, 3, 0, 0, 0, 0, BLOCK),
    c!("Pușcaș de punte", 3, 4, 2, 0, 0, 0, 0, 0),
    c!("Căpitan de punte", 4, 5, 3, 0, 0, 0, 0, 0),
    c!("Gabier cu cârlig", 3, 3, 3, 0, 0, 0, 0, HASTE),
    c!("Bombardier", 5, 6, 3, 0, 0, 0, 0, DOUBLE),
    c!("Kraken de buzunar", 7, 8, 5, 0, 0, 0, 0, 0),
    c!("Salvă de tun", 3, 3, 0, 1, 1, 0, 0, 0),
    c!("Hartă de comori", 2, 2, 0, 1, 2, 0, 0, 0),
    c!("Rom de curaj", 1, 2, 0, 1, 3, 0, 0, 0),
    c!("Torpilă", 4, 1, 0, 1, 4, 0, 0, 0),
    c!("Pansament", 2, 1, 0, 1, 5, 0, 0, 0),
    c!("Furtună", 6, 0, 0, 1, 6, 0, 0, 0),
];
const DA_PIRATI_DE_HARTIE: &[u8] = &[
    0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 6, 6, 7, 8, 8, 8, 9, 9, 10, 11, 12,
];
const DB_PIRATI_DE_HARTIE: &[u8] = &[
    0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 4, 4, 5, 5, 5, 6, 6, 7, 8, 8, 9, 9, 10, 10, 11, 13,
];
const C_EVOLUTIA_CIRCUITELOR: &[Card] = &[
    c!("Scânteie", 2, 2, 2, 0, 0, 1, 1, 0),
    c!("Fulger", 5, 4, 3, 0, 0, 1, 2, 0),
    c!("Furtună vie", 8, 7, 5, 0, 0, 1, 3, DOUBLE),
    c!("Picătură", 2, 2, 2, 0, 0, 2, 1, BLOCK),
    c!("Val", 5, 3, 4, 0, 0, 2, 2, BLOCK),
    c!("Mareea", 8, 6, 6, 0, 0, 2, 3, 0),
    c!("Pui de metal", 3, 3, 2, 0, 0, 3, 1, HASTE),
    c!("Robot de lut", 6, 5, 3, 0, 0, 3, 2, 0),
    c!("Colos de cupru", 9, 8, 5, 0, 0, 3, 3, DOUBLE),
    c!("Scurtcircuit", 2, 2, 0, 1, 1, 0, 0, 0),
    c!("Reîncărcare", 2, 2, 0, 1, 2, 0, 0, 0),
    c!("Supratensiune", 3, 2, 0, 1, 4, 0, 0, 0),
];
const DA_EVOLUTIA_CIRCUITELOR: &[u8] = &[
    0, 0, 0, 0, 1, 1, 1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 5, 6, 6, 6, 7, 7, 8, 9, 9, 9, 10, 10, 11, 11,
];
const DB_EVOLUTIA_CIRCUITELOR: &[u8] = &[
    0, 0, 0, 1, 1, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 6, 6, 6, 6, 7, 7, 7, 8, 9, 9, 9, 10, 10, 11, 11,
];
const C_ENERGIA_LUPTATORILOR: &[Card] = &[
    c!("Ucenic de arte", 1, 2, 2, 0, 0, 0, 0, 0),
    c!("Pumn de fier", 2, 3, 2, 0, 0, 0, 0, 0),
    c!("Gardian de templu", 2, 2, 4, 0, 0, 0, 0, BLOCK),
    c!("Luptător de elită", 3, 4, 3, 0, 0, 0, 0, 0),
    c!("Maestru cu dublă lovitură", 4, 4, 3, 0, 0, 0, 0, DOUBLE),
    c!("Alergător fulger", 3, 3, 2, 0, 0, 0, 0, HASTE),
    c!("Uriaș pașnic", 5, 6, 5, 0, 0, 0, 0, 0),
    c!("Val de energie", 3, 3, 0, 1, 1, 0, 0, 0),
    c!("Antrenament greu", 2, 2, 0, 1, 3, 0, 0, 0),
    c!("Meditație", 2, 2, 0, 1, 2, 0, 0, 0),
    c!("Cutremur", 4, 1, 0, 1, 4, 0, 0, 0),
    c!("Pastilă de recuperare", 2, 2, 0, 1, 5, 0, 0, 0),
];
const DA_ENERGIA_LUPTATORILOR: &[u8] = &[
    0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 7, 8, 8, 9, 9, 10, 10, 11,
];
const DB_ENERGIA_LUPTATORILOR: &[u8] = &[
    0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 5, 5, 6, 7, 7, 7, 8, 8, 8, 9, 9, 10, 11,
];
const C_CEASUL_ALB_NEGRU: &[Card] = &[
    c!("Cântăreață de scenă", 0, 2, 2, 0, 0, 0, 0, 0),
    c!("Dansatoare", 1, 3, 2, 0, 0, 0, 0, 0),
    c!("Violonist", 2, 4, 3, 0, 0, 0, 0, 0),
    c!("Corist-scut", 1, 1, 2, 0, 0, 0, 0, CANCEL),
    c!("Solistă grabnică", 2, 3, 2, 0, 0, 0, 0, HASTE),
    c!("Dirijor", 3, 5, 3, 0, 0, 0, 0, DOUBLE),
    c!("Artistă principală", 4, 7, 4, 0, 0, 0, 0, 0),
    c!("Paznic de culise", 2, 3, 4, 0, 0, 0, 0, BLOCK),
    c!("Reflector", 0, 0, 0, 2, 0, 0, 0, 0),
    c!("Ovație", 1, 2, 0, 1, 3, 0, 0, 0),
    c!("Aplauze", 2, 2, 0, 1, 2, 0, 0, 0),
    c!("Cădere de decor", 3, 3, 0, 1, 1, 0, 0, 0),
];
const DA_CEASUL_ALB_NEGRU: &[u8] = &[
    0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 7, 7, 7, 8, 8, 9,
    9, 9, 10, 10, 10, 11, 11, 11,
];
const DB_CEASUL_ALB_NEGRU: &[u8] = &[
    0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 9,
    9, 9, 10, 10, 10, 11, 11, 11,
];
const C_TREPTE_DE_AVANGARDA: &[Card] = &[
    c!("Novice", 0, 5, 4, 0, 0, 0, 0, 0),
    c!("Scutier de gardă", 0, 4, 5, 0, 0, 0, 0, 0),
    c!("Prevestitor", 0, 4, 3, 0, 0, 0, 0, TRIG),
    c!("Cadet", 0, 6, 3, 0, 0, 0, 1, 0),
    c!("Cavaler de rând", 0, 8, 4, 0, 0, 0, 1, 0),
    c!("Santinelă", 0, 7, 5, 0, 0, 0, 1, 0),
    c!("Căpitan de pluton", 0, 9, 3, 0, 0, 0, 2, 0),
    c!("Maestru de scrimă", 0, 10, 4, 0, 0, 0, 2, 0),
    c!("Gardian cu scut", 0, 8, 5, 0, 0, 0, 2, 0),
    c!("Comandant suprem", 0, 12, 3, 0, 0, 0, 3, 0),
    c!("Rege-Cavaler", 0, 13, 4, 0, 0, 0, 3, 0),
];
const DA_TREPTE_DE_AVANGARDA: &[u8] = &[
    0, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 6, 6, 6, 6, 7, 7, 7, 8, 8, 8, 9, 10,
];
const DB_TREPTE_DE_AVANGARDA: &[u8] = &[
    0, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 8, 8, 8, 9, 10,
];
const C_DOUA_ARENE_O_BAZA: &[Card] = &[
    c!("Soldat de sol", 1, 2, 2, 0, 0, 0, 0, 0),
    c!("Tanc ușor", 3, 3, 4, 0, 0, 0, 0, 0),
    c!("Gardian de poartă", 2, 2, 4, 0, 0, 0, 0, BLOCK),
    c!("Cercetaș rapid", 2, 3, 2, 0, 0, 0, 0, HASTE),
    c!("Nava mică", 2, 2, 2, 0, 0, 0, 0, AIR),
    c!("Nava de luptă", 4, 4, 4, 0, 0, 0, 0, AIR),
    c!("Escortă", 3, 2, 3, 0, 0, 0, 0, AIR | BLOCK),
    c!("Mers greu", 5, 6, 6, 0, 0, 0, 0, 0),
    c!("Crucișător", 6, 6, 7, 0, 0, 0, 0, AIR),
    c!("Bombardament", 3, 4, 0, 1, 1, 0, 0, 0),
    c!("Salvă pe bază", 3, 3, 0, 1, 4, 0, 0, 0),
    c!("Întărire", 1, 2, 0, 1, 3, 0, 0, 0),
    c!("Reparații", 2, 3, 0, 1, 5, 0, 0, 0),
];
const DA_DOUA_ARENE_O_BAZA: &[u8] = &[
    0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 7, 7, 8, 9, 9, 10, 10, 11, 12,
];
const DB_DOUA_ARENE_O_BAZA: &[u8] = &[
    0, 0, 0, 1, 1, 1, 1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 11, 11,
];
const C_SCUTURI_SI_MANA: &[Card] = &[
    c!("Flăcărușă", 1, 2, 1, 0, 0, 1, 0, 0),
    c!("Războinic de jar", 3, 4, 2, 0, 0, 1, 0, HASTE),
    c!("Dragon de lavă", 6, 8, 4, 0, 0, 1, 0, DOUBLE),
    c!("Pește de adânc", 2, 2, 2, 0, 0, 2, 0, BLOCK),
    c!("Vrăjitor de ceață", 3, 3, 3, 0, 0, 2, 0, 0),
    c!("Leviatan", 6, 7, 6, 0, 0, 2, 0, 0),
    c!("Pădurar", 2, 3, 2, 0, 0, 3, 0, 0),
    c!("Urs de mușchi", 4, 5, 4, 0, 0, 3, 0, 0),
    c!("Arbore străvechi", 5, 6, 6, 0, 0, 3, 0, BLOCK),
    c!("Scut de jar", 3, 1, 0, 1, 4, 1, 0, TRIG),
    c!("Val protector", 2, 2, 0, 1, 5, 2, 0, TRIG),
    c!("Rădăcini prinzătoare", 3, 3, 0, 1, 1, 3, 0, 0),
    c!("Gardian din scut", 3, 3, 3, 0, 0, 3, 0, TRIG | BLOCK),
];
const DA_SCUTURI_SI_MANA: &[u8] = &[
    0, 0, 0, 1, 1, 1, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 9, 9, 10, 10, 11, 12,
];
const DB_SCUTURI_SI_MANA: &[u8] = &[
    0, 0, 0, 1, 1, 1, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11,
];

pub static SPECS: &[Spec] = &[
    Spec {
        id: "pirati_de_hartie",
        mode: Mode::Pirate,
        cards: C_PIRATI_DE_HARTIE,
        decks: [DA_PIRATI_DE_HARTIE, DB_PIRATI_DE_HARTIE],
        hand: 5,
        life: 5,
        cap: 6,
        lead_pow: 4,
        turns: 26,
        start_vg: 0,
    },
    Spec {
        id: "evolutia_circuitelor",
        mode: Mode::Evo,
        cards: C_EVOLUTIA_CIRCUITELOR,
        decks: [DA_EVOLUTIA_CIRCUITELOR, DB_EVOLUTIA_CIRCUITELOR],
        hand: 5,
        life: 5,
        cap: 6,
        lead_pow: 0,
        turns: 24,
        start_vg: 0,
    },
    Spec {
        id: "energia_luptatorilor",
        mode: Mode::Charge,
        cards: C_ENERGIA_LUPTATORILOR,
        decks: [DA_ENERGIA_LUPTATORILOR, DB_ENERGIA_LUPTATORILOR],
        hand: 5,
        life: 8,
        cap: 5,
        lead_pow: 3,
        turns: 24,
        start_vg: 0,
    },
    Spec {
        id: "ceasul_alb_negru",
        mode: Mode::Clock,
        cards: C_CEASUL_ALB_NEGRU,
        decks: [DA_CEASUL_ALB_NEGRU, DB_CEASUL_ALB_NEGRU],
        hand: 5,
        life: 0,
        cap: 3,
        lead_pow: 0,
        turns: 28,
        start_vg: 0,
    },
    Spec {
        id: "trepte_de_avangarda",
        mode: Mode::Ride,
        cards: C_TREPTE_DE_AVANGARDA,
        decks: [DA_TREPTE_DE_AVANGARDA, DB_TREPTE_DE_AVANGARDA],
        hand: 5,
        life: 0,
        cap: 4,
        lead_pow: 0,
        turns: 22,
        start_vg: 0,
    },
    Spec {
        id: "doua_arene_o_baza",
        mode: Mode::Arena,
        cards: C_DOUA_ARENE_O_BAZA,
        decks: [DA_DOUA_ARENE_O_BAZA, DB_DOUA_ARENE_O_BAZA],
        hand: 6,
        life: 22,
        cap: 6,
        lead_pow: 0,
        turns: 13,
        start_vg: 0,
    },
    Spec {
        id: "scuturi_si_mana",
        mode: Mode::Mana,
        cards: C_SCUTURI_SI_MANA,
        decks: [DA_SCUTURI_SI_MANA, DB_SCUTURI_SI_MANA],
        hand: 5,
        life: 5,
        cap: 5,
        lead_pow: 0,
        turns: 22,
        start_vg: 0,
    },
];
