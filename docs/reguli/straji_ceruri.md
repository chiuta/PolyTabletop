# Străjerii cerurilor (`straji_ceruri`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play straji_ceruri --seed 1`. Sursa: `polytabletop-games/src/straji_ceruri.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Oștirea Aurorei contra Legiunea Amurgului), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 16 ture pe jucător decide scorul. Resurse: 12 cărți-sursă în fiecare pachet; o dată pe tură pui una (sursa i): energia +1 (maximum 8); energia se reface la fiecare tură. Câștigi când viața adversarului ajunge la 0 (start 20); lovitura directă scade viața cu atacul unității. Până la 6 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (gardă, zbor, primul, armură, sorbire); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: lovitură, binecuvântare, vindecare, măturare, trezire. Răsucire: toate unitățile au elan (atacă imediat). Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Oștirea Aurorei, J2 Legiunea Amurgului; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți, plus 12 cărți-sursă).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Herald Mic | 1 | 2/1 | — |
| Paznic de Nor | 1 | 1/1 | zbor |
| Lăncier Luminos | 2 | 3/2 | — |
| Cântăreț de Coastă | 2 | 1/3 | primul |
| Arhanghel Tânăr | 3 | 4/2 | sorbire |
| Străjer de Poartă | 3 | 5/2 | — |
| Războinic Înaripat | 4 | 3/5 | armură |
| Judecător de Foc | 4 | 4/5 | — |
| Serafim Albastru | 5 | 4/6 | zbor |
| Marele Străjer | 6 | 5/8 | gardă |

| Vrajă | Cost | Efect |
|---|---|---|
| Rază Sfântă | 3 | lovitură 4 |
| Aripi de Argint | 5 | binecuvântare 3 |
| Binecuvântare | 3 | vindecare 2 |
| Pământ Curățat | 4 | măturare 2 |
| Trezire | 2 | trezire 0 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
