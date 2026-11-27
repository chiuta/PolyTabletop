# Detectivii spiritelor (`detectivii_spiritelor`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play detectivii_spiritelor --seed 1`. Sursa: `polytabletop-games/src/detectivii_spiritelor.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Biroul Dincolo contra Banda Pragului), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 15 ture pe jucător decide scorul. Rezerva de energie primește +2 la fiecare tură proprie (maximum 9); ce nu cheltui se păstrează. Fiecare are 3 zone cu 7 viață; o unitate lovește o zonă (atac k zN); cu două zone distruse pierzi. Până la 5 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, zbor, primul, venin); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: îngheț, lovitură, pescuit, întoarcere în mână, trezire. Răsucire: cine ucide o unitate în luptă trage o carte. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Biroul Dincolo, J2 Banda Pragului; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Cercetător Tânăr | 1 | 1/2 | — |
| Iscoadă Mică | 1 | 1/1 | primul |
| Călăreț de Aer | 2 | 3/1 | zbor |
| Fantomă Zâmbăreață | 2 | 1/3 | zbor |
| Luptător de Stradă | 3 | 5/2 | — |
| Pistolar al Sufletelor | 3 | 1/4 | venin |
| Demon Cu Bici | 4 | 5/3 | elan |
| Maestru Îngheț | 4 | 4/4 | primul |
| Stăpânul Turnirului | 5 | 5/6 | — |
| Spiritul Primordial | 6 | 6/7 | — |

| Vrajă | Cost | Efect |
|---|---|---|
| Pumn Spiritual | 1 | îngheț 0 |
| Fulg Dublu | 3 | lovitură 4 |
| Revelație | 2 | pescuit 2 |
| Chemare Înapoi | 2 | întoarcere în mână 0 |
| Transă | 2 | trezire 0 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
