# Eroi între lumi (`eroi_intre_lumi`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play eroi_intre_lumi --seed 1`. Sursa: `polytabletop-games/src/eroi_intre_lumi.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Tărâmul Cheii contra Tărâmul Furtunii), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 15 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 9, și se reface la fiecare tură. Câștigi când viața adversarului ajunge la 0 (start 22); lovitura directă scade viața cu atacul unității. Până la 6 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (gardă, zbor, primul, armură, venin); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: măturare, lovitură, întărire, nimicire, pescuit. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Tărâmul Cheii, J2 Tărâmul Furtunii; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Cercetaș de Graniță | 1 | 2/1 | — |
| Călător Cu Hartă | 1 | 1/2 | gardă |
| Pază de Prag | 2 | 3/2 | — |
| Meșter de Aburi | 2 | 1/4 | — |
| Mag al Cheii | 3 | 3/3 | primul |
| Cavaler al Portalului | 3 | 4/2 | zbor |
| Exploratoare de Lumi | 4 | 4/4 | armură |
| Eroul Rătăcit | 4 | 3/6 | — |
| Căpitan Dintre Lumi | 5 | 4/6 | zbor |
| Marele Cartograf | 6 | 7/4 | venin |

| Vrajă | Cost | Efect |
|---|---|---|
| Val de Pietre | 3 | măturare 1 |
| Salt Între Lumi | 3 | lovitură 4 |
| Arma Aleasă | 3 | întărire 3 |
| Ucidere Țintită | 3 | nimicire 5 |
| Conexiune | 1 | pescuit 1 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
