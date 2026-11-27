# Școala darurilor (`scoala_darurilor`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play scoala_darurilor --seed 1`. Sursa: `polytabletop-games/src/scoala_darurilor.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Elevii Curții contra Clanul Trecutului), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 15 ture pe jucător decide scorul. Resurse: 11 cărți-sursă în fiecare pachet; o dată pe tură pui una (sursa i): energia +1 (maximum 8); energia se reface la fiecare tură. Fiecare are 3 zone cu 7 viață; o unitate lovește o zonă (atac k zN); cu două zone distruse pierzi. Până la 5 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, gardă, zbor, primul); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: trezire, lovitură, întărire, îngheț, întoarcere în mână. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Elevii Curții, J2 Clanul Trecutului; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți, plus 11 cărți-sursă).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Copil Cu Ochi Aurii | 1 | 1/2 | — |
| Cititor de Gânduri | 1 | 1/1 | primul |
| Elev Cu Aripi | 2 | 2/2 | elan |
| Fetiță Cu Gheață | 2 | 3/2 | — |
| Profesor de Fizică | 3 | 3/4 | gardă |
| Păzitor de Coridoare | 3 | 5/2 | — |
| Lider de Echipă | 4 | 3/6 | — |
| Mutant Rătăcit | 4 | 5/3 | primul |
| Directoarea Cu Voce | 5 | 5/5 | zbor |
| Marele Dar Ascuns | 6 | 6/6 | zbor |

| Vrajă | Cost | Efect |
|---|---|---|
| Reînnoire | 2 | trezire 0 |
| Rază Optică | 3 | lovitură 4 |
| Control Mental | 1 | întărire 1 |
| Mingea de Foc | 1 | îngheț 0 |
| Salt Prin Perete | 1 | întoarcere în mână 0 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
