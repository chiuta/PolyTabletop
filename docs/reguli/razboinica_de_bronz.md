# Războinica de bronz (`razboinica_de_bronz`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play razboinica_de_bronz --seed 1`. Sursa: `polytabletop-games/src/razboinica_de_bronz.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Taberele Nordului contra Garda Cetății), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 14 ture pe jucător decide scorul. O dată pe tură poți pune orice carte din mână drept sursă (sursa i): energia +1 (maximum 8), dar cartea se pierde; energia se reface la fiecare tură. Câștigi când viața adversarului ajunge la 0 (start 20); lovitura directă scade viața cu atacul unității. Până la 6 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, gardă, zbor, primul, armură); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: lovitură, întărire, binecuvântare, măturare, nimicire. Răsucire: cine ucide o unitate în luptă trage o carte. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Taberele Nordului, J2 Garda Cetății; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Arcaș de Munte | 1 | 2/1 | — |
| Păstor Curajos | 1 | 1/1 | zbor |
| Călăreț de Stepă | 2 | 1/4 | gardă |
| Lăncier Bronzat | 2 | 3/2 | — |
| Amazoană de Elită | 3 | 5/2 | — |
| Căpetenie Tatuată | 3 | 4/3 | — |
| Gladiator Vestit | 4 | 5/3 | primul |
| Prorociță de Piatră | 4 | 5/3 | armură |
| Generalul de Oțel | 5 | 6/4 | zbor |
| Marele Cuceritor | 6 | 7/5 | elan |

| Vrajă | Cost | Efect |
|---|---|---|
| Aruncare de Disc | 3 | lovitură 4 |
| Strigăt de Luptă | 3 | întărire 3 |
| Pavăză Întărită | 4 | binecuvântare 2 |
| Armură Nouă | 5 | măturare 3 |
| Sabia Zeilor | 4 | nimicire 9 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
