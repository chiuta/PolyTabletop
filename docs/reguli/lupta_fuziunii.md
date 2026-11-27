# Lupta fuziunii (`lupta_fuziunii`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play lupta_fuziunii --seed 1`. Sursa: `polytabletop-games/src/lupta_fuziunii.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Dojo Dinspre Soare contra Dojo Dinspre Lună), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 14 ture pe jucător decide scorul. Rezerva de energie primește +2 la fiecare tură proprie (maximum 9); ce nu cheltui se păstrează. Câștigi când viața adversarului ajunge la 0 (start 24); lovitura directă scade viața cu atacul unității. Până la 6 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, primul, armură, venin, sorbire); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: întărire, lovitură, măturare, binecuvântare, nimicire. Răsucire: loviturile directe sunt cu 1 mai tari. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Dojo Dinspre Soare, J2 Dojo Dinspre Lună; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Ucenic de Arte | 1 | 1/2 | — |
| Gonitor de Aer | 1 | 1/1 | primul |
| Luptător Roșu | 1 | 1/1 | sorbire |
| Luptător Albastru | 2 | 2/3 | — |
| Războinic Cu Aură | 2 | 2/1 | venin |
| Maestru de Ki | 3 | 3/3 | armură |
| Gemeni Fuzionați | 3 | 3/4 | — |
| Căpitan al Stelei | 4 | 4/4 | elan |
| Zeu Cu Coadă | 5 | 7/4 | — |
| Marele Războinic Fuzionat | 6 | 6/6 | elan |

| Vrajă | Cost | Efect |
|---|---|---|
| Pumn Cu Aură | 1 | întărire 1 |
| Fuziune | 3 | lovitură 4 |
| Explozie de Putere | 5 | măturare 3 |
| Bile Lucitoare | 2 | binecuvântare 1 |
| Strigăt de Ki | 4 | nimicire 9 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
