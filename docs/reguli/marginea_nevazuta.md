# Marginea nevăzută (`marginea_nevazuta`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play marginea_nevazuta --seed 1`. Sursa: `polytabletop-games/src/marginea_nevazuta.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Cercul Veghetorilor contra Societatea Uitării), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 15 ture pe jucător decide scorul. O dată pe tură poți pune orice carte din mână drept sursă (sursa i): energia +1 (maximum 8), dar cartea se pierde; energia se reface la fiecare tură. Fiecare are 3 zone cu 7 viață; o unitate lovește o zonă (atac k zN); cu două zone distruse pierzi. Până la 5 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, gardă, zbor, primul, venin); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: lovitură, îngheț, întoarcere în mână, pescuit, nimicire. Răsucire: daunele nu se vindecă niciodată. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Cercul Veghetorilor, J2 Societatea Uitării; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Martor Tulburat | 1 | 1/2 | — |
| Cititor de Semne | 1 | 1/2 | — |
| Medium Mărunt | 2 | 2/2 | elan |
| Colecționar de Ecouri | 2 | 3/2 | — |
| Pază Fără Chip | 3 | 1/4 | venin |
| Străjer al Pragului | 3 | 3/4 | — |
| Vânător de Fantome | 4 | 4/4 | zbor |
| Oracol Orb | 4 | 3/5 | primul |
| Gardian al Golului | 5 | 5/6 | gardă |
| Marele Neștiut | 6 | 5/7 | zbor |

| Vrajă | Cost | Efect |
|---|---|---|
| Fulger de Eter | 1 | lovitură 2 |
| Zid de Ceață | 2 | îngheț 0 |
| Revelație | 1 | întoarcere în mână 0 |
| Rătăcire | 3 | pescuit 3 |
| Dizolvare | 2 | nimicire 3 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
