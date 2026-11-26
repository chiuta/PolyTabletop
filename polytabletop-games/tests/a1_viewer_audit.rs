//! a1: viewer/hot-seat audit. Every playable id must be classified, so a new game
//! cannot silently skip `set_viewer` support. See docs/HOTSEAT.md for the table.
//!
//! * `HOTSEAT_SAFE` (lib.rs): `render` honours `set_viewer` (real private info).
//! * `SOLO_GOD`: one human (J1) against bots/chance with hidden info; `set_viewer(Some(Two))`
//!   appends a "VEDERE TOTALĂ" reveal-all section, any other viewer does not.
//! * `NO_HIDDEN`: nothing is private to one seat: perfect information, public dice/cards,
//!   open hands (co-op), or hidden info that is symmetric (face-down memory cards, deck order,
//!   the answer of a quiz question the computer asks) and so cannot leak by showing a seat.
use polytabletop_games::{playable, HOTSEAT_SAFE};
use std::collections::BTreeSet;

const SOLO_GOD: &[&str] = &[
    "lupii",
    "santinela",
    "consiliul",
    "valea",
    "hruba",
    "roade",
    "ceasornic",
    "farul",
];

const NO_HIDDEN: &[&str] = &[
    "tictactoe",
    "ur",
    "crazyhouse",
    "omweso",
    "twixt",
    "halma",
    "fia",
    "aceydeucey",
    "barbilliards",
    "tabula",
    "petitschevaux",
    "coganh",
    "petteia",
    "flipcup",
    "madgab",
    "sotron",
    "duckduckgoose",
    "conkers",
    "craps",
    "schocken",
    "passedix",
    "dicebaseball",
    "dropdead",
    // Q3-NO-BEGIN
    "shut_box",
    "chuck",
    "bar_dice",
    "cosmic",
    "tombola",
    "pachinko",
    "fan_tan",
    "hanoi",
    "pingpong",
    "hounds",
    "ataxx",
    "shatranj",
    "shashki",
    "o_an_quan",
    "hypergammon",
    "jacquet",
    "moultezim",
    "chu_shogi",
    "chowka_bara",
    "uckers",
    "crossword",
    "perquackey",
    "ligretto",
    "truth_dare",
    "sticla",
    "scaune_muzicale",
    "mikado",
    "kick_can",
    "mingea",
    // Q3-NO-END
    "kalah",
    "foxgeese",
    "alquerque",
    "latrunculi",
    "jungle",
    "chaturanga",
    "camelot",
    "fotbalmasa",
    "tiddlywinks",
    "joc15",
    "sudoku",
    "biliard",
    "tavli",
    "gulbara",
    "chinchirorin",
    "shipcrew",
    "beetle",
    "kismet",
    "deadmansdice",
    "snap",
    "simon",
    "baccarat",
    "hnefatafl",
    "brandubh",
    "pente",
    "havannah",
    "chinese_checkers",
    "goose",
    "keno",
    "long_nard",
    "shove_ha_penny",
    "rithmomachia",
    "pong_hau_kiki",
    "patolli",
    "duodecim_scripta",
    "jigsaw",
    "boggle",
    "kings",
    "pin_the_tail",
    "war",
    "de_a_prinselea",
    "elasticul",
    "oina",
    "loto_imagini",
    "bau_cua",
    "yacht",
    "chicago",
    "zara",
    "daldos",
    "colturi",
    "sah960",
    "impingere",
    "roiul",
    "terase",
    "cartimut",
    "sferturi",
    "inele",
    "sase",
    "morpiuri9",
    "drumul_stivelor",
    "margine",
    "izvoare",
    "trei_tipuri",
    "bilinele",
    "animale",
    "iepuri",
    "asalt",
    "dar4",
    "piramida",
    "cuburi",
    "culori_pas",
    "hochei_masa",
    "marmure",
    "blocaje",
    "zbor",
    "perechi_duel",
    "discuri",
    "armonia",
    "cai_de_curse",
    "cub2",
    "flipper_duel",
    "scuze",
    "camile",
    "itinerar",
    "oware",
    "bao",
    "seega",
    "yote",
    "hus",
    "tab",
    "senet",
    "shologuti",
    "aadupuli",
    "mangala",
    "dama",
    "ancora",
    "generala",
    "mutorere",
    "parques",
    "twoup",
    "bunco",
    "farkle",
    "crokinole",
    "chess",
    "sicbo",
    "ashtachamma",
    "draughts",
    "reversi",
    "connect4",
    "gomoku",
    "hex",
    "amazoane",
    "linii",
    "nim",
    "morris",
    "mancala",
    "go",
    "xiangqi",
    "shogi",
    "dara",
    "morabaraba",
    "sungka",
    "makruk",
    "janggi",
    "backgammon",
    "ludo",
    "snakes",
    "miniaturi",
    "yut",
    "sugoroku",
    "chaupar",
    "mosii",
    "mozaic",
    "faianta",
    "orasel",
    "congkak",
    "pallanguzhi",
    "ayo",
    "memory",
    "pachisi",
    "carrom",
    "arhipelag",
    "roata",
    "zaruri5",
    "turnul",
    "cunoastere",
    "cursa_probelor",
    "zidurile",
    "prajituri",
    "patrula",
    "reactii",
    "ordine",
    "gridiron",
    "focare",
    "strajile",
    "cenusii",
    "turnul_zorilor",
    "arhiva_abisala",
    "ruinele_cartografilor",
    "noapte_de_catifea",
    "padurea_sticlei",
    "neon_si_rune",
    "targul_ciumat",
    "insula_celor_doua_luni",
    "hotii_din_ceata",
    "crucea_de_cenusa",
    "strada_de_otel",
    "tinutul_furtunilor",
    "epava_din_inelul_sapte",
    "fiordul_gheatii_negre",
    "mlastina_strigoilor",
    "satul_lanternelor",
    "cripta",
    "apus",
    "fulger",
    "spionii",
    "mosia",
    "gospodaria",
    "cetatuia",
    "taramuri",
    "furnale",
    "poieni",
    "tinuturi",
    "sideral",
    "coroana",
    "coridor",
    "drum",
    "tripierd",
    "konane",
    "fanorona",
    "tigri",
    "surakarta",
    "hasami",
    "puncte",
    "strapungere",
    "izolare",
    "rotire",
    "coloane",
    "bile",
    "patru",
    "cupe",
    "poliedre",
    "impletitura",
    "damas",
    "renju",
    "tablut",
    "lasca",
    "ruleta",
    "bagatelle",
    "salta",
    "chaturaji",
    "sjoelen",
    "nonogram",
    "snooker",
    "parcel",
    "klondike",
    "coarda",
    "capra",
    "kim",
    "slapjack",
    "chohan",
    "balut",
    "tali",
    "crag",
    "bobing",
    // Q2 (fără informație ascunsă)
    "namethattune",
    "dame10",
    "toguz",
    "sittuyin",
    "gebeta",
    "sprouts",
    "pegsol",
    "shuffleboard",
    "yatzy",
    "ceelo",
    "oversevens",
    "skunk",
    "hazard",
    "howzat",
    "dreidel",
    "mehen",
    "cinci_pietricele",
    "jocuri_ata",
    "skittles",
    "narde",
    "trictrac",
    "tangram",
    "zipzapzop",
    "capturetheflag",
];

/// B2 (jocuri de familie, variante originale): informație publică sau simetrică (mâini deschise, zaruri/pachete publice).
const NO_HIDDEN_B2: &[&str] = &[
    "ambuteiaj",
    "banchiza",
    "balans",
    "pietre_puzzle",
    "firul",
    "genial",
    "pelerin",
    "morcovi",
    "rapide",
    "vapor",
    "lumina",
    "zgarie_nori",
    "turnuri_piatra",
    "templele",
    "caravana",
    "poteca",
    "evadare",
    "cetate_intrigi",
    "menajerie",
    "tinut_zane",
    "postalioni",
    "cazanul",
    "santier",
    "marele_bazar",
    "curierii_vraji",
    "detectiv",
    "tinuturi_negociate",
    "peisaj",
    "cariera",
    "salariu",
    "plapuma",
    "pisici",
    "triunghiuri",
    "banane",
    "lanturi",
    "bambuzeria",
];

#[test]
fn every_playable_id_is_classified() {
    let mut unclassified = Vec::new();
    let mut twice = Vec::new();
    for p in playable() {
        let n = [
            HOTSEAT_SAFE.contains(&p.id),
            SOLO_GOD.contains(&p.id),
            NO_HIDDEN.contains(&p.id) || NO_HIDDEN_B2.contains(&p.id),
        ]
        .iter()
        .filter(|b| **b)
        .count();
        if n == 0 {
            unclassified.push(p.id);
        }
        if n > 1 {
            twice.push(p.id);
        }
    }
    assert!(
        unclassified.is_empty(),
        "playable ids neither in HOTSEAT_SAFE nor SOLO_GOD nor NO_HIDDEN (implement set_viewer, \
         or document them in docs/HOTSEAT.md and this test): {unclassified:?}"
    );
    assert!(twice.is_empty(), "ids in more than one class: {twice:?}");
}

#[test]
fn lists_have_no_stale_or_duplicate_ids() {
    let ids: BTreeSet<&str> = playable().iter().map(|p| p.id).collect();
    for (name, l) in [
        ("SOLO_GOD", SOLO_GOD),
        ("NO_HIDDEN", NO_HIDDEN),
        ("NO_HIDDEN_B2", NO_HIDDEN_B2),
    ] {
        let set: BTreeSet<&str> = l.iter().copied().collect();
        assert_eq!(set.len(), l.len(), "{name} has duplicates");
        for i in l {
            assert!(ids.contains(i), "{name}: unknown id {i}");
        }
    }
}

#[test]
fn solo_god_view_reveals_only_for_second_viewer() {
    for id in SOLO_GOD {
        let p = playable().into_iter().find(|p| p.id == *id).unwrap();
        let mut g = (p.make)();
        let mut seen = false;
        for step in 0..40 {
            let mut one = g.clone_box();
            one.set_viewer(Some(polytabletop_core::Player::One));
            let mut none = g.clone_box();
            none.set_viewer(None);
            let mut two = g.clone_box();
            two.set_viewer(Some(polytabletop_core::Player::Two));
            let (r1, r0, r2) = (one.render(), none.render(), two.render());
            assert!(
                !r1.contains("VEDERE TOTALĂ") && !r0.contains("VEDERE TOTALĂ"),
                "{id}"
            );
            assert!(r2.contains("VEDERE TOTALĂ"), "{id}: god view missing");
            assert_eq!(r0, r1, "{id}: viewer None/One differ");
            assert!(r2.starts_with(&r1), "{id}: god view must only append");
            seen = true;
            let m = g.legal_moves();
            if m.is_empty() || g.outcome().is_some() {
                break;
            }
            g.play(&m[(step * 3 + 1) % m.len()]).unwrap();
        }
        assert!(seen);
    }
}
