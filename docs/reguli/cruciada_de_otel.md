# Cruciada de oțel (`cruciada_de_otel`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play cruciada_de_otel --seed 1`. Sursa: `polytabletop-games/src/cruciada_de_otel.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Oștirea Sfântă contra Hoarda Fărâmițată), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 15 ture pe jucător decide scorul. O dată pe tură poți pune orice carte din mână drept sursă (sursa i): energia +1 (maximum 8), dar cartea se pierde; energia se reface la fiecare tură. Câștigi când viața adversarului ajunge la 0 (start 24); lovitura directă scade viața cu atacul unității. Până la 6 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (gardă, primul, armură, venin, sorbire); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: nimicire, măturare, binecuvântare, lovitură, vindecare. Răsucire: daunele nu se vindecă niciodată. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Oștirea Sfântă, J2 Hoarda Fărâmițată; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Pușcaș Înnegrit | 1 | 1/2 | — |
| Penitent Cu Flacără | 2 | 1/4 | — |
| Sergent de Oțel | 2 | 3/1 | primul |
| Tunar Mecanic | 3 | 1/5 | sorbire |
| Cavaler în Armură | 3 | 2/5 | — |
| Preot de Război | 4 | 3/6 | gardă |
| Titan Mărunt | 4 | 4/5 | — |
| Căpitan Cu Sabie | 5 | 2/9 | gardă |
| Mașina Sfântă | 6 | 6/6 | armură |
| Marele Cruciat | 7 | 6/7 | venin |

| Vrajă | Cost | Efect |
|---|---|---|
| Bombă de Asediu | 3 | nimicire 5 |
| Rugăciune de Luptă | 5 | măturare 3 |
| Atac Orbital | 2 | binecuvântare 1 |
| Lovitură Dreaptă | 2 | lovitură 3 |
| Binecuvântare Aprinsă | 3 | vindecare 2 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
