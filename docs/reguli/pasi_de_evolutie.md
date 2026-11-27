# Pașii evoluției (`pasi_de_evolutie`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play pasi_de_evolutie --seed 1`. Sursa: `polytabletop-games/src/pasi_de_evolutie.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Cultul Mugurilor contra Cultul Penelor), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 15 ture pe jucător decide scorul. Resurse: 11 cărți-sursă în fiecare pachet; o dată pe tură pui una (sursa i): energia +1 (maximum 8); energia se reface la fiecare tură. Câștigi când viața adversarului ajunge la 0 (start 20); lovitura directă scade viața cu atacul unității. Până la 6 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, zbor, primul, armură, venin); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: întărire, binecuvântare, lovitură, nimicire, trezire. Răsucire: cine ucide o unitate în luptă trage o carte. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Cultul Mugurilor, J2 Cultul Penelor; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți, plus 11 cărți-sursă).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Pui de Floare | 1 | 2/1 | — |
| Spiriduș de Ramuri | 1 | 1/1 | zbor |
| Pază de Seară | 1 | 2/1 | — |
| Vrăjitoare Cu Pălărie | 2 | 2/3 | — |
| Călăreț de Cerb | 2 | 2/2 | primul |
| Dragon de Pergament | 3 | 3/3 | elan |
| Cavaler Cu Aripi | 3 | 3/3 | armură |
| Zeiță Înfloritoare | 4 | 5/2 | venin |
| Fiară Dezlănțuită | 5 | 7/4 | — |
| Zmeul Metamorfic | 6 | 6/6 | elan |

| Vrajă | Cost | Efect |
|---|---|---|
| Mugur Tăcut | 3 | întărire 3 |
| Înflorire | 4 | binecuvântare 2 |
| Lovitură Rapidă | 1 | lovitură 2 |
| Cercul Schimbării | 4 | nimicire 9 |
| Sfărâmare | 3 | trezire 0 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
