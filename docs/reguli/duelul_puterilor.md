# Duelul puterilor (`duelul_puterilor`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play duelul_puterilor --seed 1`. Sursa: `polytabletop-games/src/duelul_puterilor.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Liga Strălucirii contra Frăția Prăpastiei), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 14 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 9, și se reface la fiecare tură. Câștigi când viața adversarului ajunge la 0 (start 22); lovitura directă scade viața cu atacul unității. Până la 5 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, gardă, primul, armură, venin); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: lovitură, binecuvântare, întărire, nimicire, măturare. Răsucire: loviturile directe sunt cu 1 mai tari. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Liga Strălucirii, J2 Frăția Prăpastiei; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Cursă Rapidă | 1 | 2/1 | — |
| Mânuitor de Scut | 1 | 1/1 | elan |
| Om-Piatră | 2 | 3/2 | — |
| Zburător Solar | 2 | 4/1 | — |
| Telepat Șoptitor | 3 | 5/1 | primul |
| Arcaș de Plasmă | 3 | 4/2 | elan |
| Colos de Fier | 4 | 7/2 | — |
| Fulgerul Alb | 4 | 5/2 | venin |
| Titan Subteran | 5 | 6/5 | gardă |
| Zeul Furtunii | 6 | 7/5 | armură |

| Vrajă | Cost | Efect |
|---|---|---|
| Rafală de Energie | 1 | lovitură 2 |
| Voință de Oțel | 2 | binecuvântare 1 |
| Forță Înzecită | 3 | întărire 3 |
| Prăbușirea Puterii | 3 | nimicire 5 |
| Tăișul Zilei | 5 | măturare 3 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
