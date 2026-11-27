# Stăpânii elementelor (`stapanii_elementelor`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play stapanii_elementelor --seed 1`. Sursa: `polytabletop-games/src/stapanii_elementelor.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Pelerinii Aerului contra Garda Focului), mână ascunsă. Start: 5 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 15 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 8, și se reface la fiecare tură. Fiecare are 3 zone cu 6 viață; o unitate lovește o zonă (atac k zN); cu două zone distruse pierzi. Până la 5 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, gardă, zbor, armură); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: lovitură, măturare, îngheț, binecuvântare, vindecare. Răsucire: toate unitățile au elan (atacă imediat). Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Pelerinii Aerului, J2 Garda Focului; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Novice de Apă | 1 | 1/2 | — |
| Dansator de Vânt | 1 | 1/1 | zbor |
| Sculptor de Pietre | 2 | 1/4 | — |
| Gardian de Flăcări | 2 | 2/3 | — |
| Arcaș Din Gheață | 3 | 2/4 | zbor |
| Valuri Încolăcite | 3 | 3/4 | gardă |
| Călăreț de Furtună | 4 | 3/6 | gardă |
| Maestru de Lavă | 4 | 3/5 | elan |
| Fiara Glacială | 5 | 7/4 | — |
| Marele Stăpân | 6 | 5/7 | armură |

| Vrajă | Cost | Efect |
|---|---|---|
| Val Rece | 1 | lovitură 2 |
| Vâltoare de Vânt | 5 | măturare 3 |
| Glaciație | 2 | îngheț 0 |
| Sfera Pământului | 2 | binecuvântare 1 |
| Flacără Vindecătoare | 3 | vindecare 2 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
