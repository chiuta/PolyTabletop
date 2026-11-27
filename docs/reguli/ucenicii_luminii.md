# Ucenicii luminii (`ucenicii_luminii`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play ucenicii_luminii --seed 1`. Sursa: `polytabletop-games/src/ucenicii_luminii.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Școala Zorilor contra Școala Cețurilor), mână ascunsă. Start: 7 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 16 ture pe jucător decide scorul. Resurse: 10 cărți-sursă în fiecare pachet; o dată pe tură pui una (sursa i): energia +1 (maximum 7); energia se reface la fiecare tură. Câștigi când viața adversarului ajunge la 0 (start 18); lovitura directă scade viața cu atacul unității. Până la 5 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, gardă, zbor, primul, sorbire); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: întărire, lovitură, pescuit, vindecare, binecuvântare. Răsucire: toate unitățile au elan (atacă imediat). Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Școala Zorilor, J2 Școala Cețurilor; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți, plus 10 cărți-sursă).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Novice Zâmbitor | 1 | 2/1 | — |
| Pază Mică | 1 | 1/1 | primul |
| Copil Cu Bețișor | 1 | 1/1 | zbor |
| Elev Neastâmpărat | 2 | 3/1 | elan |
| Pilot de Antrenament | 2 | 2/3 | — |
| Ucenic Strălucitor | 3 | 3/4 | gardă |
| Maestru de Sabie | 3 | 5/2 | — |
| Gardian de Templu | 4 | 4/4 | elan |
| Cavaler Întunecat | 5 | 6/5 | — |
| Marele Îndrumător | 6 | 6/6 | sorbire |

| Vrajă | Cost | Efect |
|---|---|---|
| Antrenament Dublu | 2 | întărire 2 |
| Rază Slabă | 1 | lovitură 2 |
| Meditație Adâncă | 1 | pescuit 1 |
| Binecuvântarea Maestrului | 2 | vindecare 1 |
| Unda Liniștirii | 2 | binecuvântare 1 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
