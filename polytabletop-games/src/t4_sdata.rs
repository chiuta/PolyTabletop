//! Date pentru motorul `t4_skirm` (generat de t4gen/gen.py): un `Spec` pe joc.
#![allow(clippy::too_many_arguments)]
use crate::t4_skirm::*;

macro_rules! k {
    ($n:expr, $c:expr, $hp:expr, $mv:expr, $r:expr, $d:expr, $tn:expr, $dm:expr, $v:expr, $f:expr) => {
        Kind {
            name: $n,
            ch: $c,
            hp: $hp,
            mv: $mv,
            rng: $r,
            dice: $d,
            tn: $tn,
            dmg: $dm,
            val: $v,
            fl: $f,
        }
    };
}

const K_ULITA_NEAGRA: &[Kind] = &[
    k!("Vigilent", 'v', 4, 5, 1, 3, 4, 1, 3, LEADER),
    k!("Acrobat", 'a', 2, 6, 1, 2, 3, 1, 1, MOBILE),
    k!("Aruncător de cuțite", 'r', 2, 3, 4, 2, 3, 1, 1, MOBILE),
    k!("Căpetenie de bandă", 'c', 4, 4, 1, 2, 4, 1, 3, LEADER),
    k!("Gealat", 'g', 3, 4, 1, 2, 4, 1, 1, 0),
    k!("Pungaș", 'p', 2, 5, 1, 1, 3, 1, 1, 0),
];
const A0_ULITA_NEAGRA: &[u8] = &[0, 1, 2];
const A1_ULITA_NEAGRA: &[u8] = &[3, 4, 4, 5, 5];
const K_MINGEA_DE_PIELE: &[Kind] = &[
    k!("Atacant", 'a', 2, 6, 1, 2, 3, 1, 1, 0),
    k!("Mijlocaș", 'm', 3, 4, 1, 2, 4, 1, 1, 0),
    k!("Fundaș", 'f', 4, 3, 1, 2, 5, 1, 2, MOBILE),
    k!("Portar", 'p', 3, 3, 1, 2, 5, 1, 1, 0),
];
const A0_MINGEA_DE_PIELE: &[u8] = &[0, 0, 1, 1, 2, 3];
const A1_MINGEA_DE_PIELE: &[u8] = &[0, 0, 1, 1, 2, 3];
const K_REGATE_DE_CENUSA: &[Kind] = &[
    k!("Rege-Cenușă", 'r', 5, 3, 1, 3, 5, 1, 3, LEADER),
    k!("Gardian cu scut", 'g', 4, 2, 1, 3, 5, 1, 2, HEAVY),
    k!("Arcaș de ceață", 'a', 2, 3, 5, 2, 3, 1, 1, 0),
    k!("Vrăjitor al focului", 'v', 2, 3, 4, 2, 3, 1, 2, BLAST),
    k!("Lăncier", 'l', 3, 3, 1, 2, 4, 1, 1, 0),
];
const A0_REGATE_DE_CENUSA: &[u8] = &[0, 1, 1, 2, 3];
const A1_REGATE_DE_CENUSA: &[u8] = &[0, 1, 1, 2, 3];
const K_FIARE_SI_VRAJITORI: &[Kind] = &[
    k!(
        "Vrăjitor al turmei",
        'v',
        5,
        3,
        4,
        3,
        3,
        1,
        4,
        BLAST | LEADER
    ),
    k!("Fiară cu coarne", 'f', 6, 4, 1, 3, 5, 1, 2, HEAVY),
    k!("Lup de stâncă", 'l', 3, 6, 1, 2, 3, 1, 1, 0),
    k!("Șaman vindecător", 's', 2, 3, 1, 1, 3, 1, 1, HEAL),
    k!("Bătăuș de turmă", 'b', 3, 3, 1, 2, 4, 1, 1, 0),
];
const A0_FIARE_SI_VRAJITORI: &[u8] = &[0, 1, 2, 3, 4];
const A1_FIARE_SI_VRAJITORI: &[u8] = &[0, 1, 2, 3, 4];
const K_CADRANUL_OSTASULUI: &[Kind] = &[
    k!("Cavaler de cadran", 'c', 6, 4, 1, 4, 5, 1, 3, DIAL),
    k!("Arcaș de cadran", 'a', 4, 3, 5, 3, 3, 1, 1, DIAL),
    k!("Vrăjitor de cadran", 'v', 4, 3, 4, 3, 3, 1, 2, BLAST | DIAL),
    k!("Scutier", 's', 3, 4, 1, 2, 4, 1, 1, 0),
];
const A0_CADRANUL_OSTASULUI: &[u8] = &[0, 1, 2, 3];
const A1_CADRANUL_OSTASULUI: &[u8] = &[0, 1, 2, 3];
const K_TRUPE_DE_PLUMB: &[Kind] = &[
    k!("Pușcaș", 'p', 2, 4, 4, 2, 4, 1, 1, 0),
    k!("Mitralior", 'm', 2, 2, 6, 3, 4, 1, 2, 0),
    k!("Tanc ușor", 't', 5, 5, 5, 3, 6, 1, 3, HEAVY),
    k!("Sapator", 's', 2, 4, 1, 2, 3, 1, 1, 0),
    k!("Medic de front", 'e', 2, 3, 1, 1, 3, 1, 1, HEAL),
];
const A0_TRUPE_DE_PLUMB: &[u8] = &[0, 0, 1, 2, 3, 4];
const A1_TRUPE_DE_PLUMB: &[u8] = &[0, 0, 1, 2, 3, 4];
const K_HRUBE_SI_COMORI: &[Kind] = &[
    k!("Cavaler de peșteră", 'c', 4, 3, 1, 3, 5, 1, 1, 0),
    k!("Preot al luminii", 'p', 3, 3, 2, 2, 3, 1, 1, HEAL),
    k!("Mag al scânteii", 'm', 3, 3, 5, 2, 3, 1, 2, BLAST),
    k!("Hoț cu arc", 'h', 2, 4, 5, 2, 3, 1, 1, 0),
    k!("Orc de hrubă", 'o', 4, 3, 1, 2, 4, 1, 1, 0),
    k!("Scheletar", 's', 1, 3, 1, 2, 3, 1, 1, 0),
    k!("Trol de peșteră", 't', 6, 2, 1, 3, 4, 1, 2, 0),
];
const A0_HRUBE_SI_COMORI: &[u8] = &[0, 1, 2, 3];
const A1_HRUBE_SI_COMORI: &[u8] = &[4, 4, 5, 5, 6];
const K_INALTIMI_CU_EROI: &[Kind] = &[
    k!("Erou de vale", 'e', 4, 4, 1, 3, 4, 1, 2, 0),
    k!("Arcaș de creastă", 'a', 2, 3, 6, 2, 3, 1, 1, 0),
    k!("Titan de piatră", 't', 6, 2, 1, 4, 5, 1, 2, HEAVY),
    k!("Iscoadă", 'i', 2, 6, 1, 2, 3, 1, 1, 0),
    k!("Zburător", 'z', 2, 6, 2, 2, 3, 1, 2, FLY),
];
const A0_INALTIMI_CU_EROI: &[u8] = &[0, 1, 2, 3, 4];
const A1_INALTIMI_CU_EROI: &[u8] = &[0, 1, 2, 3, 4];
const K_LAME_SI_GHIARE: &[Kind] = &[
    k!("Mare Războinic", 'm', 5, 4, 1, 4, 5, 1, 3, LEADER),
    k!("Gheară de pădure", 'g', 3, 5, 1, 3, 3, 1, 1, 0),
    k!("Gardă de fier", 'a', 5, 2, 1, 3, 5, 1, 2, HEAVY),
    k!("Haiduc", 'h', 3, 3, 1, 3, 4, 1, 1, 0),
    k!("Cercetaș cu arc", 'c', 2, 3, 4, 2, 3, 1, 1, 0),
];
const A0_LAME_SI_GHIARE: &[u8] = &[0, 1, 2, 3, 4];
const A1_LAME_SI_GHIARE: &[u8] = &[0, 1, 2, 3, 4];
const K_STRAJERI_DE_UMBRA: &[Kind] = &[
    k!("Străjer", 's', 3, 4, 4, 3, 4, 1, 2, 0),
    k!("Căpitan de străji", 'c', 4, 4, 3, 3, 4, 1, 3, LEADER),
    k!("Ghicitor", 'g', 2, 4, 1, 1, 3, 1, 1, HEAL),
    k!("Ghoul de umbră", 'h', 2, 4, 1, 2, 3, 1, 1, 0),
    k!("Cioban al umbrei", 'i', 4, 3, 1, 3, 5, 1, 1, 0),
    k!("Spectru", 'p', 3, 5, 1, 2, 4, 1, 2, FLY),
];
const A0_STRAJERI_DE_UMBRA: &[u8] = &[0, 0, 1, 2, 0];
const A1_STRAJERI_DE_UMBRA: &[u8] = &[3, 3, 3, 3, 4, 5];
const K_PADURE_CU_MUSCHETE: &[Kind] = &[
    k!("Pușcaș de graniță", 'p', 2, 4, 5, 2, 4, 1, 2, 0),
    k!("Luptător cu topor", 'l', 3, 5, 1, 2, 3, 1, 1, 0),
    k!("Locotenent", 'o', 3, 4, 3, 2, 4, 1, 3, LEADER),
    k!("Gealat de pădure", 'g', 3, 4, 1, 2, 4, 1, 1, 0),
];
const A0_PADURE_CU_MUSCHETE: &[u8] = &[2, 0, 0, 1, 3];
const A1_PADURE_CU_MUSCHETE: &[u8] = &[2, 0, 0, 1, 3];
const K_CATUSE_SI_BAIONETE: &[Kind] = &[
    k!("Soldat de linie", 's', 2, 3, 5, 2, 4, 1, 2, 0),
    k!("Grenadier", 'g', 3, 3, 1, 3, 4, 1, 1, 0),
    k!("Ofițer", 'o', 3, 4, 3, 2, 4, 1, 3, LEADER),
    k!("Tobosar", 't', 2, 4, 1, 1, 3, 1, 1, HEAL),
];
const A0_CATUSE_SI_BAIONETE: &[u8] = &[0, 0, 0, 1, 2, 3];
const A1_CATUSE_SI_BAIONETE: &[u8] = &[0, 0, 0, 1, 2, 3];
const K_ARMATA_DE_CIMPIE: &[Kind] = &[
    k!("Batalion de linie", 'b', 4, 2, 4, 3, 4, 1, 2, 0),
    k!("Cavalerie ușoară", 'c', 3, 7, 1, 3, 4, 1, 2, 0),
    k!("Artilerie de câmp", 'a', 3, 1, 7, 4, 5, 1, 3, 0),
    k!("General", 'g', 3, 5, 1, 2, 4, 1, 3, LEADER),
];
const A0_ARMATA_DE_CIMPIE: &[u8] = &[0, 0, 0, 1, 1, 2, 3];
const A1_ARMATA_DE_CIMPIE: &[u8] = &[0, 0, 0, 1, 1, 2, 3];
const K_SULITE_SI_ARCHEBUZE: &[Kind] = &[
    k!("Pichier", 'p', 4, 2, 1, 3, 5, 1, 2, HEAVY),
    k!("Archebuzier", 'a', 3, 3, 4, 3, 4, 1, 2, 0),
    k!("Cavalerie de oțel", 'c', 4, 6, 1, 3, 5, 1, 2, 0),
    k!("Colonel", 'o', 3, 5, 1, 2, 4, 1, 3, LEADER),
];
const A0_SULITE_SI_ARCHEBUZE: &[u8] = &[0, 0, 1, 1, 2, 3];
const A1_SULITE_SI_ARCHEBUZE: &[u8] = &[0, 0, 1, 1, 2, 3];
const K_PATRULA_LINIA_ROSIE: &[Kind] = &[
    k!("Pușcaș de patrulă", 'p', 2, 4, 6, 3, 4, 1, 2, 0),
    k!("Lunetist", 'l', 2, 2, 7, 2, 3, 1, 1, 0),
    k!("Sergent", 's', 3, 4, 5, 3, 4, 1, 3, LEADER),
    k!("Gealat de infiltrare", 'g', 2, 5, 3, 2, 3, 1, 1, MOBILE),
];
const A0_PATRULA_LINIA_ROSIE: &[u8] = &[0, 0, 1, 2, 3];
const A1_PATRULA_LINIA_ROSIE: &[u8] = &[0, 0, 1, 2, 3];
const K_PISTOALE_IN_PRAF: &[Kind] = &[
    k!("Pistolar", 'p', 3, 4, 5, 2, 4, 1, 2, MOBILE),
    k!("Pușcaș cu carabină", 'u', 2, 3, 7, 2, 4, 1, 2, 0),
    k!("Șerif", 's', 4, 4, 4, 3, 4, 1, 3, LEADER),
    k!("Tâlhar", 't', 2, 5, 3, 2, 3, 1, 1, 0),
    k!("Doctor de vagon", 'd', 2, 3, 1, 1, 3, 1, 1, HEAL),
];
const A0_PISTOALE_IN_PRAF: &[u8] = &[0, 1, 2, 4];
const A1_PISTOALE_IN_PRAF: &[u8] = &[3, 3, 0, 1, 4];
const K_CARTI_PRAFUITE: &[Kind] = &[
    k!("Pistolar de noroc", 'p', 2, 4, 5, 2, 3, 1, 2, 0),
    k!("Căpitanul bandei", 'c', 3, 4, 4, 3, 4, 1, 3, LEADER),
    k!("Bătăuș de bar", 'b', 3, 3, 1, 3, 4, 1, 1, 0),
    k!("Hoț de cai", 'h', 2, 6, 1, 2, 3, 1, 1, MOBILE),
];
const A0_CARTI_PRAFUITE: &[u8] = &[1, 0, 0, 2, 3];
const A1_CARTI_PRAFUITE: &[u8] = &[1, 0, 0, 2, 3];
const K_AVENTURIERI_DE_FOILETON: &[Kind] = &[
    k!("Eroul de serie", 'e', 4, 5, 3, 3, 4, 1, 2, LEADER),
    k!("Asistentul isteț", 'a', 2, 6, 3, 2, 3, 1, 1, 0),
    k!("Atletul", 't', 3, 5, 1, 3, 4, 1, 1, 0),
    k!("Cercetașa curajoasă", 'c', 2, 4, 4, 2, 3, 1, 1, 0),
    k!("Ucigaș cu mantie", 'u', 3, 4, 1, 3, 4, 1, 1, 0),
];
const A0_AVENTURIERI_DE_FOILETON: &[u8] = &[0, 1, 2];
const A1_AVENTURIERI_DE_FOILETON: &[u8] = &[4, 3, 2, 1];
const K_CLANURI_DE_ORHIDEE: &[Kind] = &[
    k!("Maestru de clan", 'm', 4, 4, 1, 4, 5, 1, 3, LEADER),
    k!("Spadasin", 's', 3, 4, 1, 3, 4, 1, 1, 0),
    k!("Arcaș de ceremonie", 'a', 2, 3, 5, 2, 3, 1, 1, 0),
    k!("Călugăr de templu", 'c', 3, 3, 1, 2, 3, 1, 1, HEAL),
    k!("Ucenic", 'u', 2, 4, 1, 2, 3, 1, 1, 0),
];
const A0_CLANURI_DE_ORHIDEE: &[u8] = &[0, 1, 1, 2, 3];
const A1_CLANURI_DE_ORHIDEE: &[u8] = &[0, 1, 1, 2, 4, 4];
const K_SPADA_FARA_STAPAN: &[Kind] = &[
    k!("Samurai fără stăpân", 's', 4, 4, 1, 4, 5, 1, 3, LEADER),
    k!("Lăncier rătăcit", 'l', 3, 4, 1, 3, 4, 1, 1, 0),
    k!("Mercenar", 'm', 3, 3, 1, 3, 4, 1, 1, 0),
    k!("Arcaș de drum", 'a', 2, 3, 4, 2, 3, 1, 1, 0),
];
const A0_SPADA_FARA_STAPAN: &[u8] = &[0, 1, 3];
const A1_SPADA_FARA_STAPAN: &[u8] = &[2, 2, 3, 3];
const K_LEGAMANT_DE_PIATRA: &[Kind] = &[
    k!("Gardă de piatră", 'g', 5, 2, 1, 3, 5, 1, 2, HEAVY),
    k!("Arbaletier de munte", 'a', 2, 3, 5, 3, 3, 1, 1, 0),
    k!("Thane de peșteră", 't', 5, 3, 1, 3, 5, 1, 3, LEADER),
    k!("Sapator", 's', 3, 3, 1, 3, 4, 1, 1, 0),
    k!("Orc de câmp", 'o', 2, 4, 1, 2, 4, 1, 1, 0),
];
const A0_LEGAMANT_DE_PIATRA: &[u8] = &[0, 0, 1, 1, 2, 3];
const A1_LEGAMANT_DE_PIATRA: &[u8] = &[4, 4, 4, 4, 4, 3, 1];
const K_ARMURI_DE_ABUR: &[Kind] = &[
    k!("Trăgător cu abur", 't', 2, 4, 5, 2, 4, 1, 2, 0),
    k!("Mers de oțel", 'm', 6, 3, 5, 3, 5, 1, 3, HEAVY | BLAST),
    k!("Căpitan de abur", 'c', 4, 4, 3, 3, 4, 1, 3, LEADER),
    k!("Grenadier cu flacără", 'g', 3, 3, 3, 2, 3, 1, 2, BLAST),
];
const A0_ARMURI_DE_ABUR: &[u8] = &[0, 0, 1, 2, 3];
const A1_ARMURI_DE_ABUR: &[u8] = &[0, 0, 1, 2, 3];
const K_ZBOR_DE_LANCII: &[Kind] = &[
    k!("Cavalerie grea", 'c', 4, 6, 1, 4, 5, 1, 3, 0),
    k!("Pedestrime cu scut", 'p', 3, 3, 1, 3, 4, 1, 1, 0),
    k!("Arcași montați", 'a', 2, 6, 4, 2, 3, 1, 2, MOBILE),
    k!("Comandant", 'o', 3, 5, 1, 3, 4, 1, 3, LEADER),
];
const A0_ZBOR_DE_LANCII: &[u8] = &[0, 0, 1, 1, 2, 3];
const A1_ZBOR_DE_LANCII: &[u8] = &[0, 0, 1, 1, 2, 3];
const K_DOUASPREZECE_ELEMENTE: &[Kind] = &[
    k!("Element de lănci", 'e', 3, 3, 1, 3, 4, 1, 1, 0),
    k!("Element de arcași", 'l', 2, 3, 4, 2, 3, 1, 1, 0),
    k!("Element de care", 'm', 3, 5, 1, 3, 4, 1, 2, 0),
    k!("General", 'g', 3, 4, 1, 2, 4, 1, 3, LEADER),
];
const A0_DOUASPREZECE_ELEMENTE: &[u8] = &[0, 0, 1, 1, 2, 3];
const A1_DOUASPREZECE_ELEMENTE: &[u8] = &[0, 0, 1, 1, 2, 3];
const K_OASTEA_CEA_MARE: &[Kind] = &[
    k!("Pedestrime de linie", 'p', 3, 3, 1, 3, 4, 1, 1, 0),
    k!("Pedestrime ușoară", 'e', 2, 4, 3, 2, 3, 1, 1, 0),
    k!("Cavalerie de elită", 'c', 4, 6, 1, 3, 5, 1, 3, 0),
    k!("General de oaste", 'g', 4, 5, 1, 3, 4, 1, 3, LEADER),
];
const A0_OASTEA_CEA_MARE: &[u8] = &[0, 0, 0, 1, 1, 2, 3];
const A1_OASTEA_CEA_MARE: &[u8] = &[0, 0, 0, 1, 1, 2, 3];
const K_MESTESUGUL_RAZBOIULUI: &[Kind] = &[
    k!("Infanterie de elită", 'i', 4, 3, 1, 3, 5, 1, 2, 0),
    k!("Arcași ușori", 'a', 2, 4, 4, 2, 3, 1, 1, 0),
    k!("Cavalerie de război", 'c', 3, 6, 1, 3, 4, 1, 2, 0),
    k!("Strateg", 's', 3, 4, 1, 3, 4, 1, 3, LEADER),
];
const A0_MESTESUGUL_RAZBOIULUI: &[u8] = &[0, 0, 1, 1, 2, 3];
const A1_MESTESUGUL_RAZBOIULUI: &[u8] = &[0, 0, 1, 1, 2, 3];
const K_ORDINE_PE_CAROIAJ: &[Kind] = &[
    k!("Unitate de linie", 'u', 3, 3, 1, 3, 4, 1, 1, 0),
    k!("Unitate de arcași", 'n', 2, 3, 4, 2, 3, 1, 1, 0),
    k!("Unitate de care", 'i', 3, 5, 1, 3, 4, 1, 2, 0),
    k!("General", 'g', 3, 4, 1, 3, 4, 1, 3, LEADER),
];
const A0_ORDINE_PE_CAROIAJ: &[u8] = &[0, 0, 1, 1, 2, 3];
const A1_ORDINE_PE_CAROIAJ: &[u8] = &[0, 0, 1, 1, 2, 3];
const K_CAVALERI_SI_PAVAZE: &[Kind] = &[
    k!("Cavaler greu", 'c', 5, 5, 1, 4, 5, 1, 3, 0),
    k!("Pavăzar", 'p', 4, 2, 1, 2, 6, 1, 2, 0),
    k!("Arbaletier cu pavăză", 'a', 3, 3, 5, 2, 4, 1, 1, 0),
    k!("Senior", 's', 4, 4, 1, 3, 5, 1, 3, LEADER),
];
const A0_CAVALERI_SI_PAVAZE: &[u8] = &[0, 1, 1, 2, 3];
const A1_CAVALERI_SI_PAVAZE: &[u8] = &[0, 1, 1, 2, 3];
const K_MARESALI_DE_FIER: &[Kind] = &[
    k!("Regiment de bătaie", 'r', 4, 3, 1, 3, 4, 1, 2, 0),
    k!("Mașini de asediu", 'm', 3, 1, 7, 4, 4, 1, 3, 0),
    k!("Mareșal", 'a', 3, 6, 1, 2, 4, 1, 4, LEADER),
    k!("Călăreți de oțel", 'c', 4, 6, 1, 3, 5, 1, 2, 0),
];
const A0_MARESALI_DE_FIER: &[u8] = &[0, 0, 1, 2, 3, 3];
const A1_MARESALI_DE_FIER: &[u8] = &[0, 0, 1, 2, 3, 3];
const K_SALVE_PE_FRONT: &[Kind] = &[
    k!("Brigadă de linie", 'b', 5, 2, 3, 3, 4, 1, 2, 0),
    k!("Brigadă de cavalerie", 'r', 4, 5, 1, 3, 4, 1, 2, 0),
    k!("Baterie", 'a', 3, 1, 6, 4, 4, 1, 3, 0),
    k!("Corp de armată", 'c', 4, 3, 1, 2, 4, 1, 3, LEADER),
];
const A0_SALVE_PE_FRONT: &[u8] = &[0, 0, 0, 1, 2, 3];
const A1_SALVE_PE_FRONT: &[u8] = &[0, 0, 0, 1, 2, 3];
const K_FOC_SI_PULBERE: &[Kind] = &[
    k!("Brigadă cu pușcă", 'b', 4, 3, 4, 3, 4, 1, 2, 0),
    k!("Tunuri de câmp", 't', 3, 1, 6, 4, 4, 1, 3, 0),
    k!("Cavalerie de recunoaștere", 'c', 3, 6, 1, 2, 4, 1, 2, 0),
    k!("General de armată", 'g', 3, 5, 1, 3, 4, 1, 3, LEADER),
];
const A0_FOC_SI_PULBERE: &[u8] = &[0, 0, 0, 1, 2, 3];
const A1_FOC_SI_PULBERE: &[u8] = &[0, 0, 0, 1, 2, 3];
const K_PASI_DE_OTEL: &[Kind] = &[
    k!("Mers ușor", 'm', 4, 5, 5, 2, 5, 1, 2, 0),
    k!("Mers greu", 'e', 6, 3, 6, 3, 6, 1, 3, HEAVY),
    k!("Pușcaș în exoschelet", 'p', 3, 4, 4, 2, 4, 1, 2, 0),
    k!(
        "Comandant de mers",
        'c',
        5,
        4,
        5,
        3,
        5,
        1,
        3,
        LEADER | HEAVY
    ),
];
const A0_PASI_DE_OTEL: &[u8] = &[0, 1, 2, 3];
const A1_PASI_DE_OTEL: &[u8] = &[0, 1, 2, 3];
const K_ESCADRILA_CERULUI_ROSU: &[Kind] = &[
    k!("Vânător", 'v', 3, 6, 3, 3, 4, 1, 2, FLY),
    k!("Bombardier", 'b', 5, 4, 2, 4, 5, 1, 3, FLY | BLAST),
    k!("Escortă", 'e', 2, 7, 3, 2, 3, 1, 2, FLY | MOBILE),
    k!(
        "Comandant de escadrilă",
        'c',
        4,
        6,
        3,
        3,
        4,
        1,
        3,
        FLY | LEADER
    ),
];
const A0_ESCADRILA_CERULUI_ROSU: &[u8] = &[0, 0, 1, 3, 2];
const A1_ESCADRILA_CERULUI_ROSU: &[u8] = &[0, 0, 1, 3, 2];
const K_LUPTA_IN_NORI: &[Kind] = &[
    k!("Biplan agil", 'b', 2, 7, 2, 3, 4, 1, 2, FLY),
    k!("Monoplan", 'm', 3, 6, 3, 3, 4, 1, 2, FLY),
    k!("Tunar de coadă", 't', 3, 5, 4, 2, 4, 1, 2, FLY),
    k!("As", 'a', 4, 7, 3, 4, 5, 1, 3, FLY | LEADER),
];
const A0_LUPTA_IN_NORI: &[u8] = &[0, 1, 2, 3];
const A1_LUPTA_IN_NORI: &[u8] = &[0, 1, 2, 3];
const K_ECHIPAJUL_RATACIT: &[Kind] = &[
    k!("Pilot", 'p', 3, 4, 5, 2, 4, 1, 2, 0),
    k!("Tehnician", 't', 2, 4, 3, 2, 3, 1, 1, HEAL),
    k!("Mercenar", 'm', 4, 4, 1, 3, 4, 1, 2, 0),
    k!("Bandit stelar", 'b', 2, 4, 4, 2, 4, 1, 1, 0),
    k!("Dronă sălbatică", 'd', 1, 6, 1, 2, 3, 1, 1, 0),
];
const A0_ECHIPAJUL_RATACIT: &[u8] = &[0, 1, 2];
const A1_ECHIPAJUL_RATACIT: &[u8] = &[3, 3, 4, 4, 3];
const K_PLANETA_CU_SPINI: &[Kind] = &[
    k!("Infanterist colonial", 'i', 3, 4, 5, 3, 4, 1, 2, 0),
    k!("Sergent colonial", 's', 4, 4, 4, 3, 4, 1, 3, LEADER),
    k!("Flăcărar", 'f', 3, 3, 3, 2, 3, 1, 2, BLAST),
    k!("Vierme de spini", 'v', 2, 5, 1, 2, 4, 1, 1, 0),
    k!("Zburător de spini", 'z', 2, 6, 1, 2, 3, 1, 2, FLY),
    k!("Colos de spini", 'c', 7, 3, 1, 4, 5, 1, 4, HEAVY),
];
const A0_PLANETA_CU_SPINI: &[u8] = &[0, 0, 1, 2];
const A1_PLANETA_CU_SPINI: &[u8] = &[3, 3, 3, 4, 4, 5];
const K_ZEII_DE_LUT: &[Kind] = &[
    k!("Zeu al furtunii", 'z', 6, 4, 4, 4, 5, 1, 4, LEADER | BLAST),
    k!("Erou de lut", 'e', 5, 4, 1, 3, 5, 1, 2, 0),
    k!("Zeiță a vânătorii", 'i', 3, 3, 6, 3, 3, 1, 1, 0),
    k!("Monstru de piatră", 'm', 7, 3, 1, 4, 5, 1, 3, HEAVY),
    k!("Preoteasă", 'p', 3, 3, 1, 1, 3, 1, 1, HEAL),
];
const A0_ZEII_DE_LUT: &[u8] = &[0, 1, 2, 4];
const A1_ZEII_DE_LUT: &[u8] = &[0, 1, 3, 2];
const K_ASALT_PE_STATIA_VECHE: &[Kind] = &[
    k!("Rebel cu pușcă", 'r', 3, 4, 5, 3, 4, 1, 2, 0),
    k!("Mecanic de stație", 'm', 3, 3, 3, 2, 3, 1, 1, HEAL),
    k!("Lunetist", 'l', 2, 3, 7, 3, 3, 1, 1, 0),
    k!("Soldat de asalt", 's', 3, 3, 5, 2, 4, 1, 2, 0),
    k!("Robot de pază", 'o', 5, 3, 5, 3, 5, 1, 3, HEAVY),
];
const A0_ASALT_PE_STATIA_VECHE: &[u8] = &[0, 1, 2];
const A1_ASALT_PE_STATIA_VECHE: &[u8] = &[3, 3, 3, 4];
const K_CULOARE_DE_OTEL: &[Kind] = &[
    k!("Gardian în armură", 'g', 4, 3, 5, 3, 5, 1, 2, HEAVY),
    k!(
        "Sergent în armură",
        's',
        4,
        3,
        5,
        3,
        5,
        1,
        3,
        LEADER | HEAVY
    ),
    k!("Roi de ghiare", 'r', 2, 6, 1, 2, 3, 1, 1, 0),
    k!("Patriarh al roiului", 'p', 5, 5, 1, 3, 4, 1, 3, 0),
];
const A0_CULOARE_DE_OTEL: &[u8] = &[0, 0, 1, 0, 0];
const A1_CULOARE_DE_OTEL: &[u8] = &[2, 2, 2, 2, 2, 2, 3];

pub static SPECS: &[Spec] = &[
    Spec {
        id: "ulita_neagra",
        w: 8,
        h: 8,
        grid: Grid::Sq8,
        pat: Pat::Ruins,
        kinds: K_ULITA_NEAGRA,
        army: [A0_ULITA_NEAGRA, A1_ULITA_NEAGRA],
        ap: Ap::Fix(3),
        roll: Roll::Count,
        obj: Obj::Leader,
        rounds: 14,
        goal: 0,
        mods: FLANK | COUNTER | ZOC,
        loot: 0,
    },
    Spec {
        id: "mingea_de_piele",
        w: 7,
        h: 9,
        grid: Grid::Sq8,
        pat: Pat::Open,
        kinds: K_MINGEA_DE_PIELE,
        army: [A0_MINGEA_DE_PIELE, A1_MINGEA_DE_PIELE],
        ap: Ap::Fix(3),
        roll: Roll::Opp,
        obj: Obj::Ball,
        rounds: 16,
        goal: 3,
        mods: PUSH | ZOC,
        loot: 0,
    },
    Spec {
        id: "regate_de_cenusa",
        w: 8,
        h: 8,
        grid: Grid::Sq8,
        pat: Pat::Ridge,
        kinds: K_REGATE_DE_CENUSA,
        army: [A0_REGATE_DE_CENUSA, A1_REGATE_DE_CENUSA],
        ap: Ap::Fix(3),
        roll: Roll::Count,
        obj: Obj::Leader,
        rounds: 14,
        goal: 0,
        mods: CHARGE | FLANK,
        loot: 0,
    },
    Spec {
        id: "fiare_si_vrajitori",
        w: 9,
        h: 8,
        grid: Grid::Hex,
        pat: Pat::Rubble,
        kinds: K_FIARE_SI_VRAJITORI,
        army: [A0_FIARE_SI_VRAJITORI, A1_FIARE_SI_VRAJITORI],
        ap: Ap::Fix(3),
        roll: Roll::Count,
        obj: Obj::Leader,
        rounds: 14,
        goal: 0,
        mods: CHARGE | PUSH,
        loot: 0,
    },
    Spec {
        id: "cadranul_ostasului",
        w: 9,
        h: 8,
        grid: Grid::Hex,
        pat: Pat::Hills,
        kinds: K_CADRANUL_OSTASULUI,
        army: [A0_CADRANUL_OSTASULUI, A1_CADRANUL_OSTASULUI],
        ap: Ap::Fix(3),
        roll: Roll::Count,
        obj: Obj::Elim,
        rounds: 12,
        goal: 0,
        mods: FLANK,
        loot: 0,
    },
    Spec {
        id: "trupe_de_plumb",
        w: 10,
        h: 8,
        grid: Grid::Sq8,
        pat: Pat::Rubble,
        kinds: K_TRUPE_DE_PLUMB,
        army: [A0_TRUPE_DE_PLUMB, A1_TRUPE_DE_PLUMB],
        ap: Ap::Fix(3),
        roll: Roll::Sum,
        obj: Obj::Hold,
        rounds: 10,
        goal: 8,
        mods: SUPPRESS | MORALE,
        loot: 0,
    },
    Spec {
        id: "hrube_si_comori",
        w: 8,
        h: 8,
        grid: Grid::Sq4,
        pat: Pat::Corridors,
        kinds: K_HRUBE_SI_COMORI,
        army: [A0_HRUBE_SI_COMORI, A1_HRUBE_SI_COMORI],
        ap: Ap::Fix(3),
        roll: Roll::Opp,
        obj: Obj::Loot,
        rounds: 16,
        goal: 3,
        mods: COUNTER,
        loot: 4,
    },
    Spec {
        id: "inaltimi_cu_eroi",
        w: 9,
        h: 9,
        grid: Grid::Hex,
        pat: Pat::Hills,
        kinds: K_INALTIMI_CU_EROI,
        army: [A0_INALTIMI_CU_EROI, A1_INALTIMI_CU_EROI],
        ap: Ap::Fix(3),
        roll: Roll::Count,
        obj: Obj::Elim,
        rounds: 14,
        goal: 0,
        mods: FLANK,
        loot: 0,
    },
    Spec {
        id: "lame_si_ghiare",
        w: 8,
        h: 8,
        grid: Grid::Sq8,
        pat: Pat::Pillars,
        kinds: K_LAME_SI_GHIARE,
        army: [A0_LAME_SI_GHIARE, A1_LAME_SI_GHIARE],
        ap: Ap::Fix(3),
        roll: Roll::Opp,
        obj: Obj::Leader,
        rounds: 14,
        goal: 0,
        mods: COUNTER | FLANK | PUSH,
        loot: 0,
    },
    Spec {
        id: "strajeri_de_umbra",
        w: 8,
        h: 10,
        grid: Grid::Sq4,
        pat: Pat::Corridors,
        kinds: K_STRAJERI_DE_UMBRA,
        army: [A0_STRAJERI_DE_UMBRA, A1_STRAJERI_DE_UMBRA],
        ap: Ap::Fix(3),
        roll: Roll::Count,
        obj: Obj::Breach,
        rounds: 12,
        goal: 3,
        mods: ZOC,
        loot: 0,
    },
    Spec {
        id: "padure_cu_muschete",
        w: 9,
        h: 8,
        grid: Grid::Sq8,
        pat: Pat::Rubble,
        kinds: K_PADURE_CU_MUSCHETE,
        army: [A0_PADURE_CU_MUSCHETE, A1_PADURE_CU_MUSCHETE],
        ap: Ap::Roll,
        roll: Roll::Count,
        obj: Obj::Hold,
        rounds: 12,
        goal: 7,
        mods: MORALE | SUPPRESS,
        loot: 0,
    },
    Spec {
        id: "catuse_si_baionete",
        w: 10,
        h: 8,
        grid: Grid::Sq8,
        pat: Pat::Open,
        kinds: K_CATUSE_SI_BAIONETE,
        army: [A0_CATUSE_SI_BAIONETE, A1_CATUSE_SI_BAIONETE],
        ap: Ap::Cmd,
        roll: Roll::Count,
        obj: Obj::Elim,
        rounds: 12,
        goal: 0,
        mods: MORALE | CHARGE | ZOC,
        loot: 0,
    },
    Spec {
        id: "armata_de_cimpie",
        w: 12,
        h: 8,
        grid: Grid::Sq8,
        pat: Pat::Open,
        kinds: K_ARMATA_DE_CIMPIE,
        army: [A0_ARMATA_DE_CIMPIE, A1_ARMATA_DE_CIMPIE],
        ap: Ap::Scale,
        roll: Roll::Sum,
        obj: Obj::Hold,
        rounds: 12,
        goal: 9,
        mods: CHARGE | FLANK | MORALE,
        loot: 0,
    },
    Spec {
        id: "sulite_si_archebuze",
        w: 12,
        h: 8,
        grid: Grid::Sq8,
        pat: Pat::Open,
        kinds: K_SULITE_SI_ARCHEBUZE,
        army: [A0_SULITE_SI_ARCHEBUZE, A1_SULITE_SI_ARCHEBUZE],
        ap: Ap::Scale,
        roll: Roll::Count,
        obj: Obj::Leader,
        rounds: 12,
        goal: 0,
        mods: CHARGE | FLANK | ZOC,
        loot: 0,
    },
    Spec {
        id: "patrula_linia_rosie",
        w: 9,
        h: 9,
        grid: Grid::Sq8,
        pat: Pat::Ruins,
        kinds: K_PATRULA_LINIA_ROSIE,
        army: [A0_PATRULA_LINIA_ROSIE, A1_PATRULA_LINIA_ROSIE],
        ap: Ap::Fix(4),
        roll: Roll::Count,
        obj: Obj::Loot,
        rounds: 12,
        goal: 2,
        mods: SUPPRESS,
        loot: 3,
    },
    Spec {
        id: "pistoale_in_praf",
        w: 8,
        h: 8,
        grid: Grid::Sq8,
        pat: Pat::Rubble,
        kinds: K_PISTOALE_IN_PRAF,
        army: [A0_PISTOALE_IN_PRAF, A1_PISTOALE_IN_PRAF],
        ap: Ap::Fix(3),
        roll: Roll::Count,
        obj: Obj::Loot,
        rounds: 14,
        goal: 3,
        mods: COUNTER,
        loot: 3,
    },
    Spec {
        id: "carti_prafuite",
        w: 9,
        h: 8,
        grid: Grid::Sq8,
        pat: Pat::Pillars,
        kinds: K_CARTI_PRAFUITE,
        army: [A0_CARTI_PRAFUITE, A1_CARTI_PRAFUITE],
        ap: Ap::Roll,
        roll: Roll::Opp,
        obj: Obj::Loot,
        rounds: 12,
        goal: 3,
        mods: PUSH,
        loot: 3,
    },
    Spec {
        id: "aventurieri_de_foileton",
        w: 8,
        h: 8,
        grid: Grid::Sq8,
        pat: Pat::Ruins,
        kinds: K_AVENTURIERI_DE_FOILETON,
        army: [A0_AVENTURIERI_DE_FOILETON, A1_AVENTURIERI_DE_FOILETON],
        ap: Ap::Fix(3),
        roll: Roll::Sum,
        obj: Obj::Loot,
        rounds: 14,
        goal: 4,
        mods: PUSH | FLANK,
        loot: 5,
    },
    Spec {
        id: "clanuri_de_orhidee",
        w: 8,
        h: 8,
        grid: Grid::Sq8,
        pat: Pat::Pillars,
        kinds: K_CLANURI_DE_ORHIDEE,
        army: [A0_CLANURI_DE_ORHIDEE, A1_CLANURI_DE_ORHIDEE],
        ap: Ap::Fix(3),
        roll: Roll::Opp,
        obj: Obj::Leader,
        rounds: 14,
        goal: 0,
        mods: COUNTER | FLANK,
        loot: 0,
    },
    Spec {
        id: "spada_fara_stapan",
        w: 7,
        h: 7,
        grid: Grid::Sq4,
        pat: Pat::Pillars,
        kinds: K_SPADA_FARA_STAPAN,
        army: [A0_SPADA_FARA_STAPAN, A1_SPADA_FARA_STAPAN],
        ap: Ap::Fix(2),
        roll: Roll::Opp,
        obj: Obj::Elim,
        rounds: 16,
        goal: 0,
        mods: COUNTER | PUSH,
        loot: 0,
    },
    Spec {
        id: "legamant_de_piatra",
        w: 10,
        h: 8,
        grid: Grid::Sq8,
        pat: Pat::Ridge,
        kinds: K_LEGAMANT_DE_PIATRA,
        army: [A0_LEGAMANT_DE_PIATRA, A1_LEGAMANT_DE_PIATRA],
        ap: Ap::Scale,
        roll: Roll::Count,
        obj: Obj::Hold,
        rounds: 12,
        goal: 8,
        mods: MORALE | CHARGE | FLANK,
        loot: 0,
    },
    Spec {
        id: "armuri_de_abur",
        w: 9,
        h: 9,
        grid: Grid::Sq8,
        pat: Pat::Ruins,
        kinds: K_ARMURI_DE_ABUR,
        army: [A0_ARMURI_DE_ABUR, A1_ARMURI_DE_ABUR],
        ap: Ap::Fix(3),
        roll: Roll::Sum,
        obj: Obj::Elim,
        rounds: 12,
        goal: 0,
        mods: SUPPRESS,
        loot: 0,
    },
    Spec {
        id: "zbor_de_lancii",
        w: 12,
        h: 8,
        grid: Grid::Sq8,
        pat: Pat::Open,
        kinds: K_ZBOR_DE_LANCII,
        army: [A0_ZBOR_DE_LANCII, A1_ZBOR_DE_LANCII],
        ap: Ap::Scale,
        roll: Roll::Sum,
        obj: Obj::Hold,
        rounds: 12,
        goal: 9,
        mods: CHARGE | FLANK,
        loot: 0,
    },
    Spec {
        id: "douasprezece_elemente",
        w: 10,
        h: 8,
        grid: Grid::Sq4,
        pat: Pat::Open,
        kinds: K_DOUASPREZECE_ELEMENTE,
        army: [A0_DOUASPREZECE_ELEMENTE, A1_DOUASPREZECE_ELEMENTE],
        ap: Ap::Roll,
        roll: Roll::Count,
        obj: Obj::Leader,
        rounds: 16,
        goal: 0,
        mods: CHARGE | FLANK | ZOC,
        loot: 0,
    },
    Spec {
        id: "oastea_cea_mare",
        w: 12,
        h: 9,
        grid: Grid::Sq4,
        pat: Pat::Ridge,
        kinds: K_OASTEA_CEA_MARE,
        army: [A0_OASTEA_CEA_MARE, A1_OASTEA_CEA_MARE],
        ap: Ap::Roll,
        roll: Roll::Sum,
        obj: Obj::Leader,
        rounds: 14,
        goal: 0,
        mods: CHARGE | FLANK | MORALE | ZOC,
        loot: 0,
    },
    Spec {
        id: "mestesugul_razboiului",
        w: 10,
        h: 8,
        grid: Grid::Sq8,
        pat: Pat::Ridge,
        kinds: K_MESTESUGUL_RAZBOIULUI,
        army: [A0_MESTESUGUL_RAZBOIULUI, A1_MESTESUGUL_RAZBOIULUI],
        ap: Ap::Fix(3),
        roll: Roll::Sum,
        obj: Obj::Hold,
        rounds: 12,
        goal: 8,
        mods: FLANK | CHARGE,
        loot: 0,
    },
    Spec {
        id: "ordine_pe_caroiaj",
        w: 12,
        h: 8,
        grid: Grid::Sq4,
        pat: Pat::Open,
        kinds: K_ORDINE_PE_CAROIAJ,
        army: [A0_ORDINE_PE_CAROIAJ, A1_ORDINE_PE_CAROIAJ],
        ap: Ap::Roll,
        roll: Roll::Opp,
        obj: Obj::Hold,
        rounds: 14,
        goal: 8,
        mods: CHARGE | ZOC,
        loot: 0,
    },
    Spec {
        id: "cavaleri_si_pavaze",
        w: 10,
        h: 8,
        grid: Grid::Sq8,
        pat: Pat::Open,
        kinds: K_CAVALERI_SI_PAVAZE,
        army: [A0_CAVALERI_SI_PAVAZE, A1_CAVALERI_SI_PAVAZE],
        ap: Ap::Fix(3),
        roll: Roll::Sum,
        obj: Obj::Leader,
        rounds: 12,
        goal: 0,
        mods: CHARGE | COUNTER,
        loot: 0,
    },
    Spec {
        id: "maresali_de_fier",
        w: 12,
        h: 8,
        grid: Grid::Sq8,
        pat: Pat::Pillars,
        kinds: K_MARESALI_DE_FIER,
        army: [A0_MARESALI_DE_FIER, A1_MARESALI_DE_FIER],
        ap: Ap::Cmd,
        roll: Roll::Count,
        obj: Obj::Leader,
        rounds: 12,
        goal: 0,
        mods: CHARGE | FLANK,
        loot: 0,
    },
    Spec {
        id: "salve_pe_front",
        w: 11,
        h: 9,
        grid: Grid::Hex,
        pat: Pat::Open,
        kinds: K_SALVE_PE_FRONT,
        army: [A0_SALVE_PE_FRONT, A1_SALVE_PE_FRONT],
        ap: Ap::Scale,
        roll: Roll::Count,
        obj: Obj::Hold,
        rounds: 12,
        goal: 8,
        mods: MORALE | FLANK,
        loot: 0,
    },
    Spec {
        id: "foc_si_pulbere",
        w: 11,
        h: 8,
        grid: Grid::Sq4,
        pat: Pat::Rubble,
        kinds: K_FOC_SI_PULBERE,
        army: [A0_FOC_SI_PULBERE, A1_FOC_SI_PULBERE],
        ap: Ap::Fix(3),
        roll: Roll::Sum,
        obj: Obj::Elim,
        rounds: 12,
        goal: 0,
        mods: MORALE | SUPPRESS,
        loot: 0,
    },
    Spec {
        id: "pasi_de_otel",
        w: 10,
        h: 9,
        grid: Grid::Hex,
        pat: Pat::Pillars,
        kinds: K_PASI_DE_OTEL,
        army: [A0_PASI_DE_OTEL, A1_PASI_DE_OTEL],
        ap: Ap::Fix(3),
        roll: Roll::Sum,
        obj: Obj::Leader,
        rounds: 12,
        goal: 0,
        mods: ZOC,
        loot: 0,
    },
    Spec {
        id: "escadrila_cerului_rosu",
        w: 10,
        h: 10,
        grid: Grid::Hex,
        pat: Pat::Open,
        kinds: K_ESCADRILA_CERULUI_ROSU,
        army: [A0_ESCADRILA_CERULUI_ROSU, A1_ESCADRILA_CERULUI_ROSU],
        ap: Ap::Fix(3),
        roll: Roll::Count,
        obj: Obj::Leader,
        rounds: 12,
        goal: 0,
        mods: 0,
        loot: 0,
    },
    Spec {
        id: "lupta_in_nori",
        w: 9,
        h: 9,
        grid: Grid::Hex,
        pat: Pat::Open,
        kinds: K_LUPTA_IN_NORI,
        army: [A0_LUPTA_IN_NORI, A1_LUPTA_IN_NORI],
        ap: Ap::Fix(2),
        roll: Roll::Opp,
        obj: Obj::Elim,
        rounds: 14,
        goal: 0,
        mods: FLANK | MORALE,
        loot: 0,
    },
    Spec {
        id: "echipajul_ratacit",
        w: 8,
        h: 8,
        grid: Grid::Sq8,
        pat: Pat::Rubble,
        kinds: K_ECHIPAJUL_RATACIT,
        army: [A0_ECHIPAJUL_RATACIT, A1_ECHIPAJUL_RATACIT],
        ap: Ap::Fix(3),
        roll: Roll::Count,
        obj: Obj::Loot,
        rounds: 14,
        goal: 3,
        mods: SUPPRESS,
        loot: 4,
    },
    Spec {
        id: "planeta_cu_spini",
        w: 9,
        h: 9,
        grid: Grid::Hex,
        pat: Pat::Ruins,
        kinds: K_PLANETA_CU_SPINI,
        army: [A0_PLANETA_CU_SPINI, A1_PLANETA_CU_SPINI],
        ap: Ap::Fix(3),
        roll: Roll::Count,
        obj: Obj::Hold,
        rounds: 12,
        goal: 7,
        mods: MORALE | ZOC,
        loot: 0,
    },
    Spec {
        id: "zeii_de_lut",
        w: 9,
        h: 8,
        grid: Grid::Hex,
        pat: Pat::Hills,
        kinds: K_ZEII_DE_LUT,
        army: [A0_ZEII_DE_LUT, A1_ZEII_DE_LUT],
        ap: Ap::Fix(3),
        roll: Roll::Opp,
        obj: Obj::Leader,
        rounds: 14,
        goal: 0,
        mods: PUSH | FLANK,
        loot: 0,
    },
    Spec {
        id: "asalt_pe_statia_veche",
        w: 10,
        h: 9,
        grid: Grid::Sq8,
        pat: Pat::Corridors,
        kinds: K_ASALT_PE_STATIA_VECHE,
        army: [A0_ASALT_PE_STATIA_VECHE, A1_ASALT_PE_STATIA_VECHE],
        ap: Ap::Fix(3),
        roll: Roll::Sum,
        obj: Obj::Loot,
        rounds: 14,
        goal: 3,
        mods: SUPPRESS,
        loot: 4,
    },
    Spec {
        id: "culoare_de_otel",
        w: 7,
        h: 11,
        grid: Grid::Sq4,
        pat: Pat::Corridors,
        kinds: K_CULOARE_DE_OTEL,
        army: [A0_CULOARE_DE_OTEL, A1_CULOARE_DE_OTEL],
        ap: Ap::Fix(3),
        roll: Roll::Count,
        obj: Obj::Breach,
        rounds: 16,
        goal: 2,
        mods: ZOC,
        loot: 0,
    },
];
