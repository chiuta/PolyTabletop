# Legiunile frânte (`legiuni_frante`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play legiuni_frante --seed 1`. Sursa: `polytabletop-games/src/legiuni_frante.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Corpul de Fier contra Războinicii Roșii), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 15 ture pe jucător decide scorul. Resurse: 11 cărți-sursă în fiecare pachet; o dată pe tură pui una (sursa i): energia +1 (maximum 8); energia se reface la fiecare tură. Câștigi când viața adversarului ajunge la 0 (start 22); lovitura directă scade viața cu atacul unității. Până la 6 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, gardă, primul, armură, venin); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: măturare, lovitură, binecuvântare, nimicire, întărire. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Corpul de Fier, J2 Războinicii Roșii; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți, plus 11 cărți-sursă).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Recrut Înnegrit | 1 | 2/1 | — |
| Pușcaș de Tranșee | 1 | 2/1 | — |
| Sapator de Cratere | 2 | 2/3 | gardă |
| Sergent Șchiop | 2 | 3/1 | elan |
| Tanc Ușor | 3 | 4/3 | — |
| Comando de Noapte | 3 | 3/2 | venin |
| Hoplit Mecanic | 4 | 5/3 | armură |
| Căpitan de Oțel | 4 | 6/3 | — |
| Fortăreață Mobilă | 5 | 6/4 | primul |
| Marele Colos | 6 | 6/6 | elan |

| Vrajă | Cost | Efect |
|---|---|---|
| Bombardament | 5 | măturare 3 |
| Salvă Țintită | 3 | lovitură 4 |
| Ordin de Atac | 5 | binecuvântare 3 |
| Redută | 4 | nimicire 9 |
| Reîntărire | 3 | întărire 3 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
