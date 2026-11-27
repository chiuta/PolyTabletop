//! Specificații (lista C2): jocuri cooperative, variante proprii. Generat de work17/c2gen/gen.py.
use crate::c2_betray::*;
use crate::c2_quest::*;
use crate::c2_route::*;
use crate::c2_siege::*;
use crate::c2_surv::*;

pub static SATUL_STAFIILOR: SSpec = SSpec {
    title: "Satul Stafiilor",
    lanes: &["nord", "est", "sud", "vest"],
    heroes: ["Preotul", "Vânătoarea"],
    start: 5,
    base: 10,
    rounds: 8,
    boss_hp: 9,
    kill_goal: 0,
    spawn: 55,
    hand: 4,
    bonus: [0, 1],
    wall: false,
    heal: true,
    freeze: false,
    speed: 1,
};
pub static PAZNICII_PADURII: SSpec = SSpec {
    title: "Paznicii Pădurii",
    lanes: &["luncă", "pădure", "stâncă", "râu", "deal"],
    heroes: ["Lupul", "Vulturul"],
    start: 4,
    base: 9,
    rounds: 7,
    boss_hp: 0,
    kill_goal: 11,
    spawn: 60,
    hand: 4,
    bonus: [1, 0],
    wall: false,
    heal: true,
    freeze: true,
    speed: 1,
};
pub static SCUTIERII_PASULUI: SSpec = SSpec {
    title: "Scutierii Pasului",
    lanes: &["pasul", "podul", "poarta"],
    heroes: ["Arcașa", "Scutierul"],
    start: 6,
    base: 7,
    rounds: 9,
    boss_hp: 10,
    kill_goal: 0,
    spawn: 50,
    hand: 4,
    bonus: [1, 0],
    wall: true,
    heal: false,
    freeze: false,
    speed: 1,
};
pub static ALARMA_NAVA: SSpec = SSpec {
    title: "Alarma pe Navă",
    lanes: &["punte", "motoare", "scuturi", "arme"],
    heroes: ["Pilotul", "Inginera"],
    start: 3,
    base: 12,
    rounds: 7,
    boss_hp: 0,
    kill_goal: 12,
    spawn: 65,
    hand: 5,
    bonus: [0, 0],
    wall: true,
    heal: true,
    freeze: false,
    speed: 1,
};
pub static AUTOMATE_CONTRA_ROIULUI: SSpec = SSpec {
    title: "Automatele contra Roiului",
    lanes: &["stânga", "centru", "dreapta"],
    heroes: ["Tunarul", "Reparatorul"],
    start: 5,
    base: 9,
    rounds: 8,
    boss_hp: 11,
    kill_goal: 0,
    spawn: 70,
    hand: 5,
    bonus: [1, 0],
    wall: true,
    heal: true,
    freeze: false,
    speed: 1,
};
pub static TEMNITA_CLEPSIDREI: SSpec = SSpec {
    title: "Temnița Clepsidrei",
    lanes: &["coridor", "sală", "gârlă"],
    heroes: ["Cavalerul", "Magiciana"],
    start: 3,
    base: 6,
    rounds: 6,
    boss_hp: 8,
    kill_goal: 0,
    spawn: 70,
    hand: 5,
    bonus: [0, 1],
    wall: false,
    heal: false,
    freeze: true,
    speed: 1,
};
pub static CAMPIONII_URBEI: SSpec = SSpec {
    title: "Campionii Orașului",
    lanes: &["bancă", "port", "turn", "metrou"],
    heroes: ["Zburătorul", "Scutul"],
    start: 5,
    base: 10,
    rounds: 8,
    boss_hp: 12,
    kill_goal: 0,
    spawn: 50,
    hand: 5,
    bonus: [0, 1],
    wall: false,
    heal: true,
    freeze: false,
    speed: 1,
};
pub static ACADEMIA_VRAJILOR: SSpec = SSpec {
    title: "Academia Vrăjilor",
    lanes: &["bibliotecă", "seră", "turn", "pivniță"],
    heroes: ["Ucenicul", "Ucenica"],
    start: 4,
    base: 10,
    rounds: 7,
    boss_hp: 8,
    kill_goal: 0,
    spawn: 55,
    hand: 5,
    bonus: [0, 0],
    wall: false,
    heal: true,
    freeze: true,
    speed: 1,
};
pub static APARAREA_PLANETEI: SSpec = SSpec {
    title: "Apărarea Planetei",
    lanes: &["America", "Europa", "Asia", "Africa", "Oceania"],
    heroes: ["Comandantul", "Cercetașa"],
    start: 5,
    base: 11,
    rounds: 9,
    boss_hp: 0,
    kill_goal: 14,
    spawn: 60,
    hand: 4,
    bonus: [1, 0],
    wall: true,
    heal: false,
    freeze: true,
    speed: 1,
};
pub static BASTIONUL_CENUSIU: SSpec = SSpec {
    title: "Bastionul Cenușiu",
    lanes: &["poarta", "zid", "turn", "curte"],
    heroes: ["Paznicul", "Vrăjitoarea"],
    start: 6,
    base: 8,
    rounds: 10,
    boss_hp: 14,
    kill_goal: 0,
    spawn: 45,
    hand: 4,
    bonus: [0, 1],
    wall: true,
    heal: true,
    freeze: false,
    speed: 1,
};
pub static UMBRE_PESTE_CETATE: SSpec = SSpec {
    title: "Umbre peste Cetate",
    lanes: &["turnir", "pădure", "lac", "drum"],
    heroes: ["Cavalerul Alb", "Cavalerul Roșu"],
    start: 5,
    base: 8,
    rounds: 9,
    boss_hp: 10,
    kill_goal: 0,
    spawn: 50,
    hand: 4,
    bonus: [0, 0],
    wall: true,
    heal: false,
    freeze: true,
    speed: 1,
};
pub static CALEA_FLACARII: SSpec = SSpec {
    title: "Calea Flăcării",
    lanes: &["poteca", "mlaștina", "peștera", "muntele"],
    heroes: ["Purtătorul", "Însoțitorul"],
    start: 6,
    base: 9,
    rounds: 10,
    boss_hp: 13,
    kill_goal: 0,
    spawn: 40,
    hand: 4,
    bonus: [0, 1],
    wall: false,
    heal: true,
    freeze: true,
    speed: 1,
};
pub static TEMNITA_UNUI_PACHET: QSpec = QSpec {
    title: "Temnița dintr-un Pachet",
    heroes: ["Luptătorul", "Hoața"],
    kinds: ["Forță", "Iscusință", "Minte"],
    steps: 5,
    hp: 8,
    hand: 4,
    pow_base: 5,
    boss_pow: 11,
    rest: false,
    loot: true,
    clock: 0,
};
pub static HOINARII_CU_ZARURI: QSpec = QSpec {
    title: "Hoinarii cu Zaruri",
    heroes: ["Gheară", "Scânteie"],
    kinds: ["Forță", "Viteză", "Magie"],
    steps: 6,
    hp: 9,
    hand: 4,
    pow_base: 5,
    boss_pow: 12,
    rest: true,
    loot: false,
    clock: 0,
};
pub static EROI_CU_FISE: QSpec = QSpec {
    title: "Eroii cu Fișe",
    heroes: ["Elfa", "Piticul"],
    kinds: ["Corp", "Dibăcie", "Voință"],
    steps: 6,
    hp: 8,
    hand: 5,
    pow_base: 4,
    boss_pow: 12,
    rest: true,
    loot: true,
    clock: 0,
};
pub static TINUTUL_CETELOR: QSpec = QSpec {
    title: "Ținutul Cetelor",
    heroes: ["Căpitana", "Arcașul"],
    kinds: ["Luptă", "Grație", "Duh"],
    steps: 5,
    hp: 7,
    hand: 4,
    pow_base: 6,
    boss_pow: 13,
    rest: false,
    loot: false,
    clock: 0,
};
pub static JUCARIILE_CURAJOASE: QSpec = QSpec {
    title: "Jucăriile Curajoase",
    heroes: ["Ursulețul", "Iepurașul"],
    kinds: ["Curaj", "Joacă", "Poveste"],
    steps: 5,
    hp: 10,
    hand: 4,
    pow_base: 4,
    boss_pow: 10,
    rest: true,
    loot: false,
    clock: 0,
};
pub static CONACUL_INFECTAT: QSpec = QSpec {
    title: "Conacul Infectat",
    heroes: ["Polițista", "Studentul"],
    kinds: ["Tragere", "Fugă", "Reflecție"],
    steps: 6,
    hp: 7,
    hand: 4,
    pow_base: 5,
    boss_pow: 13,
    rest: false,
    loot: true,
    clock: 14,
};
pub static ULTIMA_SUPRAVIETUITOARE: QSpec = QSpec {
    title: "Ultima Supraviețuitoare",
    heroes: ["Tânăra", "Paznicul"],
    kinds: ["Luptă", "Ascundere", "Ingeniozitate"],
    steps: 5,
    hp: 6,
    hand: 4,
    pow_base: 5,
    boss_pow: 12,
    rest: true,
    loot: false,
    clock: 13,
};
pub static CRIPTA_CORBILOR: QSpec = QSpec {
    title: "Cripta Corbilor",
    heroes: ["Clericul", "Ranger-ul"],
    kinds: ["Sabie", "Rugă", "Săgeată"],
    steps: 6,
    hp: 8,
    hand: 4,
    pow_base: 5,
    boss_pow: 13,
    rest: true,
    loot: true,
    clock: 0,
};
pub static CONTINENTUL_NECUNOSCUT: QSpec = QSpec {
    title: "Continentul Necunoscut",
    heroes: ["Exploratoarea", "Geologul"],
    kinds: ["Rezistență", "Orientare", "Cunoaștere"],
    steps: 8,
    hp: 9,
    hand: 5,
    pow_base: 4,
    boss_pow: 11,
    rest: true,
    loot: true,
    clock: 18,
};
pub static ZEII_ADORMITI: QSpec = QSpec {
    title: "Zeii Adormiți",
    heroes: ["Căpitana", "Cartograful"],
    kinds: ["Cutezanță", "Marinărie", "Înțelepciune"],
    steps: 7,
    hp: 8,
    hand: 4,
    pow_base: 5,
    boss_pow: 12,
    rest: true,
    loot: false,
    clock: 16,
};
pub static APELE_UITATE: QSpec = QSpec {
    title: "Apele Uitate",
    heroes: ["Pirata", "Cârmaciul"],
    kinds: ["Sabie", "Cârmă", "Vorbă"],
    steps: 6,
    hp: 8,
    hand: 4,
    pow_base: 4,
    boss_pow: 11,
    rest: false,
    loot: true,
    clock: 15,
};
pub static CANTECUL_HOINAR: QSpec = QSpec {
    title: "Cântecul Hoinar",
    heroes: ["Menestrelul", "Fantoma"],
    kinds: ["Melodie", "Umbră", "Amintire"],
    steps: 5,
    hp: 7,
    hand: 4,
    pow_base: 5,
    boss_pow: 11,
    rest: true,
    loot: false,
    clock: 0,
};
pub static ORASUL_CU_CEATA: QSpec = QSpec {
    title: "Orașul cu Ceață",
    heroes: ["Anchetatorul", "Bibliotecara"],
    kinds: ["Curaj", "Rațiune", "Credință"],
    steps: 7,
    hp: 7,
    hand: 4,
    pow_base: 5,
    boss_pow: 14,
    rest: false,
    loot: false,
    clock: 17,
};
pub static DETECTIVII_MAGIEI: QSpec = QSpec {
    title: "Detectivii Magiei",
    heroes: ["Vrăjitorul", "Ofițera"],
    kinds: ["Putere", "Anchetă", "Vorbă"],
    steps: 5,
    hp: 8,
    hand: 5,
    pow_base: 5,
    boss_pow: 11,
    rest: false,
    loot: true,
    clock: 12,
};
pub static VASUL_PUSTIIT: QSpec = QSpec {
    title: "Vasul Pustiit",
    heroes: ["Medicul", "Inginerul"],
    kinds: ["Foc", "Tehnică", "Liniște"],
    steps: 7,
    hp: 6,
    hand: 4,
    pow_base: 5,
    boss_pow: 13,
    rest: true,
    loot: true,
    clock: 16,
};
pub static IARNA_TABEREI: VSpec = VSpec {
    title: "Iarna Taberei",
    heroes: ["Vânătorul", "Medica"],
    res: ["hrană", "materiale", "combustibil"],
    locs: [
        ("Magazinul", 0, 2),
        ("Depozitul", 1, 3),
        ("Școala", 1, 2),
        ("Benzinăria", 2, 3),
    ],
    pop: 4,
    goal: 8,
    rounds: 10,
    morale: 6,
    crises: &[(0, 4, 1), (2, 4, 1), (1, 5, 2), (0, 5, 2)],
    project: "Baricada",
};
pub static NEAMUL_PIETREI: VSpec = VSpec {
    title: "Neamul Pietrei",
    heroes: ["Culegătoarea", "Vânătorul"],
    res: ["carne", "piatră", "lemn"],
    locs: [
        ("Savana", 0, 3),
        ("Cariera", 1, 2),
        ("Pădurea", 2, 2),
        ("Râul", 0, 2),
    ],
    pop: 5,
    goal: 7,
    rounds: 10,
    morale: 7,
    crises: &[(0, 4, 1), (2, 4, 1), (1, 4, 1), (0, 5, 2)],
    project: "Tabăra",
};
pub static INSULA_APELOR_URCATE: VSpec = VSpec {
    title: "Insula Apelor Urcate",
    heroes: ["Arhitecta", "Marinarul"],
    res: ["grâne", "blocuri", "funii"],
    locs: [
        ("Ferma", 0, 2),
        ("Cariera", 1, 3),
        ("Pădurea", 2, 2),
        ("Portul", 1, 2),
    ],
    pop: 4,
    goal: 9,
    rounds: 11,
    morale: 5,
    crises: &[(1, 4, 1), (2, 4, 1), (1, 6, 2), (0, 5, 1)],
    project: "Digul",
};
pub static NAVA_FARA_CAPITAN: VSpec = VSpec {
    title: "Nava fără Căpitan",
    heroes: ["Ofițerul", "Inginera"],
    res: ["aer", "energie", "piese"],
    locs: [
        ("Punte", 0, 2),
        ("Reactor", 1, 3),
        ("Atelier", 2, 3),
        ("Hangar", 1, 2),
    ],
    pop: 4,
    goal: 8,
    rounds: 9,
    morale: 6,
    crises: &[(1, 4, 1), (0, 4, 1), (2, 5, 2), (1, 5, 2)],
    project: "Saltul",
};
pub static ULTIMELE_ANIMALE: VSpec = VSpec {
    title: "Ultimele Animale",
    heroes: ["Biologul", "Ranger-ul"],
    res: ["hrană", "fonduri", "teren"],
    locs: [
        ("Savana", 0, 2),
        ("Donații", 1, 3),
        ("Rezervația", 2, 2),
        ("Laboratorul", 1, 2),
    ],
    pop: 3,
    goal: 7,
    rounds: 9,
    morale: 6,
    crises: &[(1, 4, 1), (0, 4, 1), (2, 4, 1), (1, 5, 2)],
    project: "Rezervația",
};
pub static ZORII_CLIMEI: VSpec = VSpec {
    title: "Zorii Climei",
    heroes: ["Delegata", "Inventatorul"],
    res: ["energie", "fonduri", "voință"],
    locs: [
        ("Rețeaua", 0, 2),
        ("Banca", 1, 3),
        ("Piața", 2, 2),
        ("Laboratorul", 1, 2),
    ],
    pop: 4,
    goal: 9,
    rounds: 11,
    morale: 6,
    crises: &[(0, 4, 1), (1, 5, 1), (2, 4, 1), (1, 6, 2)],
    project: "Tranziția",
};
pub static CALEA_ASCUNSA: RSpec = RSpec {
    title: "Calea Ascunsă",
    heroes: ["Conductoarea", "Gazda"],
    n: 12,
    chord: 0,
    src: 4,
    goal: 4,
    lim: 2,
    rate: 5,
    ramp: 4,
    fly: false,
    node: "Popas",
};
pub static FOCARE_DUPA_CATASTROFA: RSpec = RSpec {
    title: "Focare după Catastrofă",
    heroes: ["Medicul", "Curierul"],
    n: 14,
    chord: 2,
    src: 5,
    goal: 4,
    lim: 3,
    rate: 5,
    ramp: 3,
    fly: true,
    node: "Oraș",
};
pub static FURTUNA_INTERZISA: RSpec = RSpec {
    title: "Furtuna Interzisă",
    heroes: ["Electriciana", "Alpinistul"],
    n: 10,
    chord: 1,
    src: 3,
    goal: 3,
    lim: 2,
    rate: 4,
    ramp: 3,
    fly: false,
    node: "Stâlp",
};
pub static CONACUL_TRADARII: BSpec = BSpec {
    title: "Conacul Trădării",
    rooms: 18,
    omens: 4,
    len: 6,
    beast_hp: 4,
    hero_hp: 5,
    reveal_betrays: true,
};
pub static FLOTA_INFILTRATA: BSpec = BSpec {
    title: "Flota Infiltrată",
    rooms: 16,
    omens: 3,
    len: 7,
    beast_hp: 5,
    hero_hp: 6,
    reveal_betrays: false,
};
pub static COMPLOTUL_DIN_UMBRA: BSpec = BSpec {
    title: "Complotul din Umbră",
    rooms: 20,
    omens: 5,
    len: 5,
    beast_hp: 6,
    hero_hp: 7,
    reveal_betrays: false,
};
#[cfg(test)]
const SPEC_IDS: &[&str] = &[
    "satul_stafiilor",
    "paznicii_padurii",
    "scutierii_pasului",
    "alarma_nava",
    "automate_contra_roiului",
    "temnita_clepsidrei",
    "campionii_urbei",
    "academia_vrajilor",
    "apararea_planetei",
    "bastionul_cenusiu",
    "umbre_peste_cetate",
    "calea_flacarii",
    "temnita_unui_pachet",
    "hoinarii_cu_zaruri",
    "eroi_cu_fise",
    "tinutul_cetelor",
    "jucariile_curajoase",
    "conacul_infectat",
    "ultima_supravietuitoare",
    "cripta_corbilor",
    "continentul_necunoscut",
    "zeii_adormiti",
    "apele_uitate",
    "cantecul_hoinar",
    "orasul_cu_ceata",
    "detectivii_magiei",
    "vasul_pustiit",
    "iarna_taberei",
    "neamul_pietrei",
    "insula_apelor_urcate",
    "nava_fara_capitan",
    "ultimele_animale",
    "zorii_climei",
    "calea_ascunsa",
    "focare_dupa_catastrofa",
    "furtuna_interzisa",
    "conacul_tradarii",
    "flota_infiltrata",
    "complotul_din_umbra",
];

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn satul_stafiilor_plays() {
        crate::brand_util::check(|s| crate::c2_siege::S::with_seed(&SATUL_STAFIILOR, s), 3000);
    }
    #[test]
    fn paznicii_padurii_plays() {
        crate::brand_util::check(
            |s| crate::c2_siege::S::with_seed(&PAZNICII_PADURII, s),
            3000,
        );
    }
    #[test]
    fn scutierii_pasului_plays() {
        crate::brand_util::check(
            |s| crate::c2_siege::S::with_seed(&SCUTIERII_PASULUI, s),
            3000,
        );
    }
    #[test]
    fn alarma_nava_plays() {
        crate::brand_util::check(|s| crate::c2_siege::S::with_seed(&ALARMA_NAVA, s), 3000);
    }
    #[test]
    fn automate_contra_roiului_plays() {
        crate::brand_util::check(
            |s| crate::c2_siege::S::with_seed(&AUTOMATE_CONTRA_ROIULUI, s),
            3000,
        );
    }
    #[test]
    fn temnita_clepsidrei_plays() {
        crate::brand_util::check(
            |s| crate::c2_siege::S::with_seed(&TEMNITA_CLEPSIDREI, s),
            3000,
        );
    }
    #[test]
    fn campionii_urbei_plays() {
        crate::brand_util::check(|s| crate::c2_siege::S::with_seed(&CAMPIONII_URBEI, s), 3000);
    }
    #[test]
    fn academia_vrajilor_plays() {
        crate::brand_util::check(
            |s| crate::c2_siege::S::with_seed(&ACADEMIA_VRAJILOR, s),
            3000,
        );
    }
    #[test]
    fn apararea_planetei_plays() {
        crate::brand_util::check(
            |s| crate::c2_siege::S::with_seed(&APARAREA_PLANETEI, s),
            3000,
        );
    }
    #[test]
    fn bastionul_cenusiu_plays() {
        crate::brand_util::check(
            |s| crate::c2_siege::S::with_seed(&BASTIONUL_CENUSIU, s),
            3000,
        );
    }
    #[test]
    fn umbre_peste_cetate_plays() {
        crate::brand_util::check(
            |s| crate::c2_siege::S::with_seed(&UMBRE_PESTE_CETATE, s),
            3000,
        );
    }
    #[test]
    fn calea_flacarii_plays() {
        crate::brand_util::check(|s| crate::c2_siege::S::with_seed(&CALEA_FLACARII, s), 3000);
    }
    #[test]
    fn temnita_unui_pachet_plays() {
        crate::brand_util::check(
            |s| crate::c2_quest::Q::with_seed(&TEMNITA_UNUI_PACHET, s),
            3000,
        );
    }
    #[test]
    fn hoinarii_cu_zaruri_plays() {
        crate::brand_util::check(
            |s| crate::c2_quest::Q::with_seed(&HOINARII_CU_ZARURI, s),
            3000,
        );
    }
    #[test]
    fn eroi_cu_fise_plays() {
        crate::brand_util::check(|s| crate::c2_quest::Q::with_seed(&EROI_CU_FISE, s), 3000);
    }
    #[test]
    fn tinutul_cetelor_plays() {
        crate::brand_util::check(|s| crate::c2_quest::Q::with_seed(&TINUTUL_CETELOR, s), 3000);
    }
    #[test]
    fn jucariile_curajoase_plays() {
        crate::brand_util::check(
            |s| crate::c2_quest::Q::with_seed(&JUCARIILE_CURAJOASE, s),
            3000,
        );
    }
    #[test]
    fn conacul_infectat_plays() {
        crate::brand_util::check(
            |s| crate::c2_quest::Q::with_seed(&CONACUL_INFECTAT, s),
            3000,
        );
    }
    #[test]
    fn ultima_supravietuitoare_plays() {
        crate::brand_util::check(
            |s| crate::c2_quest::Q::with_seed(&ULTIMA_SUPRAVIETUITOARE, s),
            3000,
        );
    }
    #[test]
    fn cripta_corbilor_plays() {
        crate::brand_util::check(|s| crate::c2_quest::Q::with_seed(&CRIPTA_CORBILOR, s), 3000);
    }
    #[test]
    fn continentul_necunoscut_plays() {
        crate::brand_util::check(
            |s| crate::c2_quest::Q::with_seed(&CONTINENTUL_NECUNOSCUT, s),
            3000,
        );
    }
    #[test]
    fn zeii_adormiti_plays() {
        crate::brand_util::check(|s| crate::c2_quest::Q::with_seed(&ZEII_ADORMITI, s), 3000);
    }
    #[test]
    fn apele_uitate_plays() {
        crate::brand_util::check(|s| crate::c2_quest::Q::with_seed(&APELE_UITATE, s), 3000);
    }
    #[test]
    fn cantecul_hoinar_plays() {
        crate::brand_util::check(|s| crate::c2_quest::Q::with_seed(&CANTECUL_HOINAR, s), 3000);
    }
    #[test]
    fn orasul_cu_ceata_plays() {
        crate::brand_util::check(|s| crate::c2_quest::Q::with_seed(&ORASUL_CU_CEATA, s), 3000);
    }
    #[test]
    fn detectivii_magiei_plays() {
        crate::brand_util::check(
            |s| crate::c2_quest::Q::with_seed(&DETECTIVII_MAGIEI, s),
            3000,
        );
    }
    #[test]
    fn vasul_pustiit_plays() {
        crate::brand_util::check(|s| crate::c2_quest::Q::with_seed(&VASUL_PUSTIIT, s), 3000);
    }
    #[test]
    fn iarna_taberei_plays() {
        crate::brand_util::check(|s| crate::c2_surv::V::with_seed(&IARNA_TABEREI, s), 3000);
    }
    #[test]
    fn neamul_pietrei_plays() {
        crate::brand_util::check(|s| crate::c2_surv::V::with_seed(&NEAMUL_PIETREI, s), 3000);
    }
    #[test]
    fn insula_apelor_urcate_plays() {
        crate::brand_util::check(
            |s| crate::c2_surv::V::with_seed(&INSULA_APELOR_URCATE, s),
            3000,
        );
    }
    #[test]
    fn nava_fara_capitan_plays() {
        crate::brand_util::check(
            |s| crate::c2_surv::V::with_seed(&NAVA_FARA_CAPITAN, s),
            3000,
        );
    }
    #[test]
    fn ultimele_animale_plays() {
        crate::brand_util::check(|s| crate::c2_surv::V::with_seed(&ULTIMELE_ANIMALE, s), 3000);
    }
    #[test]
    fn zorii_climei_plays() {
        crate::brand_util::check(|s| crate::c2_surv::V::with_seed(&ZORII_CLIMEI, s), 3000);
    }
    #[test]
    fn calea_ascunsa_plays() {
        crate::brand_util::check(|s| crate::c2_route::R::with_seed(&CALEA_ASCUNSA, s), 3000);
    }
    #[test]
    fn focare_dupa_catastrofa_plays() {
        crate::brand_util::check(
            |s| crate::c2_route::R::with_seed(&FOCARE_DUPA_CATASTROFA, s),
            3000,
        );
    }
    #[test]
    fn furtuna_interzisa_plays() {
        crate::brand_util::check(
            |s| crate::c2_route::R::with_seed(&FURTUNA_INTERZISA, s),
            3000,
        );
    }
    #[test]
    fn conacul_tradarii_plays() {
        crate::brand_util::check(
            |s| crate::c2_betray::Bt::with_seed(&CONACUL_TRADARII, s),
            3000,
        );
    }
    #[test]
    fn flota_infiltrata_plays() {
        crate::brand_util::check(
            |s| crate::c2_betray::Bt::with_seed(&FLOTA_INFILTRATA, s),
            3000,
        );
    }
    #[test]
    fn complotul_din_umbra_plays() {
        crate::brand_util::check(
            |s| crate::c2_betray::Bt::with_seed(&COMPLOTUL_DIN_UMBRA, s),
            3000,
        );
    }

    /// AI-ul (J1) contra Jocului: rata de victorii trebuie să fie nenulă în ansamblu și Jocul să mai câștige.
    #[test]
    fn ai_team_win_rates() {
        use polytabletop_core::{Outcome, Player};
        polytabletop_core::ai::set_time_budget_override_ms(Some(6));
        let mut total = 0;
        let mut wins = 0;
        for p in crate::playable()
            .into_iter()
            .filter(|p| p.category == "Cooperativ" && SPEC_IDS.contains(&p.id))
        {
            let mut w = 0;
            for seed in 1..=10u64 {
                let mut g = (p.make)();
                let mut n = 0;
                while g.outcome().is_none() && n < 3000 {
                    let m = g.ai_move(seed * 31 + n).expect("AI fara mutare");
                    assert!(g.legal_moves().contains(&m));
                    g.play(&m).unwrap();
                    n += 1;
                }
                if g.outcome() == Some(Outcome::Win(Player::One)) {
                    w += 1;
                }
            }
            eprintln!("RATE {} {}/6", p.id, w);
            total += 10;
            wins += w;
        }
        assert!(wins > 0 && wins < total, "rate {wins}/{total}");
    }
}
