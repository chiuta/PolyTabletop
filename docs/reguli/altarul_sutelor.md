# Altarul sutelor (`altarul_sutelor`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play altarul_sutelor --seed 1`. Sursa: `polytabletop-games/src/altarul_sutelor.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Cultul Lumânării contra Ordinul Cenușii), mână ascunsă. Start: 5 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 16 ture pe jucător decide scorul. Resurse: 12 cărți-sursă în fiecare pachet; o dată pe tură pui una (sursa i): energia +1 (maximum 8); energia se reface la fiecare tură. Fiecare are 5 sigilii: o lovitură directă rupe 1 sigiliu (2 dacă atacul e cel puțin 4); fără sigilii, următoarea lovitură directă câștigă. Până la 5 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (gardă, zbor, armură, venin); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: lovitură, vindecare, nimicire, măturare, pescuit. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Cultul Lumânării, J2 Ordinul Cenușii; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți, plus 12 cărți-sursă).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Acolit Tăcut | 1 | 1/2 | — |
| Slujitor de Ceară | 2 | 2/3 | — |
| Cântăreț de Veghe | 2 | 3/2 | — |
| Strigoi Mărunt | 3 | 3/4 | gardă |
| Preot Mohorât | 3 | 3/2 | venin |
| Cavaler Osuar | 4 | 4/4 | zbor |
| Prorocul Orb | 4 | 4/3 | venin |
| Stăpân de Cripte | 5 | 4/6 | armură |
| Umbra Altarului | 6 | 5/8 | — |
| Hecatomba Vie | 7 | 5/10 | gardă |

| Vrajă | Cost | Efect |
|---|---|---|
| Ofrandă Neagră | 2 | lovitură 3 |
| Rug Curățitor | 2 | vindecare 1 |
| Vindecare Amară | 2 | nimicire 3 |
| Sunetul Clopotului | 4 | măturare 2 |
| Jertfă Mare | 2 | pescuit 2 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
