# Păpușile dorinței (`papusile_dorintei`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play papusile_dorintei --seed 1`. Sursa: `polytabletop-games/src/papusile_dorintei.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Atelierul Porțelanului contra Cutia Cu Chei), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 16 ture pe jucător decide scorul. O dată pe tură poți pune orice carte din mână drept sursă (sursa i): energia +1 (maximum 8), dar cartea se pierde; energia se reface la fiecare tură. Fiecare are 3 zone cu 8 viață; o unitate lovește o zonă (atac k zN); cu două zone distruse pierzi. Până la 5 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (gardă, zbor, armură, venin); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: lovitură, binecuvântare, întoarcere în mână, vindecare, îngheț. Răsucire: loviturile directe sunt cu 1 mai tari. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Atelierul Porțelanului, J2 Cutia Cu Chei; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Păpușă de Cârpă | 1 | 1/2 | — |
| Păpușă-Cheie | 2 | 1/4 | gardă |
| Păpușă Cu Panglică | 2 | 1/4 | gardă |
| Păpușă de Sticlă | 3 | 2/4 | zbor |
| Păpușă Cu Voal | 3 | 1/5 | armură |
| Păpușă Veghetoare | 4 | 3/6 | — |
| Păpușă Cu Ceas | 4 | 3/6 | — |
| Păpușă Fără Chip | 5 | 4/5 | venin |
| Păpușă Regină | 6 | 6/6 | zbor |
| Marea Păpușă Dorită | 7 | 7/8 | — |

| Vrajă | Cost | Efect |
|---|---|---|
| Dorință Mică | 1 | lovitură 2 |
| Fir de Păpușar | 4 | binecuvântare 2 |
| Retragere în Cutie | 2 | întoarcere în mână 0 |
| Dar Ascuns | 1 | vindecare 1 |
| Dorință Rea | 1 | îngheț 0 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
