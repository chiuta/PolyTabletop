# Recif zburdalnic (`recif_zburdalnic`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play recif_zburdalnic --seed 1`. Sursa: `polytabletop-games/src/recif_zburdalnic.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Clanul Coralilor contra Clanul Meduzei), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 14 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 9, și se reface la fiecare tură. Fiecare are 5 sigilii: o lovitură directă rupe 1 sigiliu (2 dacă atacul e cel puțin 4); fără sigilii, următoarea lovitură directă câștigă. Până la 6 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, gardă, zbor, venin); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: îngheț, pescuit, întărire, lovitură, întoarcere în mână. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Clanul Coralilor, J2 Clanul Meduzei; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Puiet de Pește | 1 | 2/1 | — |
| Crab Zgâlțâit | 1 | 1/2 | — |
| Melc Încet | 1 | 1/1 | elan |
| Calmar Artist | 2 | 1/3 | elan |
| Sepie Cu Aripi | 2 | 3/2 | — |
| Rechin Pitic | 3 | 2/3 | venin |
| Crevete Boxer | 3 | 4/3 | gardă |
| Țestoasă Bătrână | 4 | 6/3 | — |
| Caracatiță Mare | 5 | 6/4 | zbor |
| Balena Cântătoare | 6 | 6/6 | zbor |

| Vrajă | Cost | Efect |
|---|---|---|
| Val Scurt | 1 | îngheț 0 |
| Bule de Săpun | 3 | pescuit 3 |
| Ghidușie | 1 | întărire 1 |
| Furtună de Nisip | 3 | lovitură 4 |
| Vâltoare | 1 | întoarcere în mână 0 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
