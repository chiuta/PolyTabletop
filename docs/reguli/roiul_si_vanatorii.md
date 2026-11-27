# Roiul și vânătorii (`roiul_si_vanatorii`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play roiul_si_vanatorii --seed 1`. Sursa: `polytabletop-games/src/roiul_si_vanatorii.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Roiul din Peșteri contra Vânătorii de Trofee), mână ascunsă. Start: 5 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 14 ture pe jucător decide scorul. Rezerva de energie primește +2 la fiecare tură proprie (maximum 9); ce nu cheltui se păstrează. Fiecare are 3 zone cu 7 viață; o unitate lovește o zonă (atac k zN); cu două zone distruse pierzi. Până la 6 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, zbor, armură, venin); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: măturare, lovitură, binecuvântare, întărire, pescuit. Răsucire: toate unitățile au elan (atacă imediat). Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Roiul din Peșteri, J2 Vânătorii de Trofee; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Larvă Flămândă | 1 | 1/2 | — |
| Pândar Lacom | 1 | 1/2 | — |
| Alergător Cu Gheare | 1 | 1/1 | elan |
| Gonitor Subteran | 2 | 1/3 | zbor |
| Soldat Chitinos | 2 | 1/3 | armură |
| Vânător Cu Mască | 3 | 5/2 | — |
| Ucigaș Tăcut | 3 | 3/2 | venin |
| Matcă Mică | 4 | 6/3 | — |
| Căpetenie Cu Coarne | 5 | 7/3 | zbor |
| Regina Roiului | 6 | 7/5 | elan |

| Vrajă | Cost | Efect |
|---|---|---|
| Sfâșiere | 5 | măturare 3 |
| Tălpi Sub Pământ | 3 | lovitură 4 |
| Eclozare | 4 | binecuvântare 2 |
| Rețea de Plasă | 1 | întărire 1 |
| Capcană Termică | 1 | pescuit 1 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
