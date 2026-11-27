# Căutătorii de cripte (`cautatorii_de_cripte`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play cautatorii_de_cripte --seed 1`. Sursa: `polytabletop-games/src/cautatorii_de_cripte.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Expediția Aurie contra Șacalii Cenușii), mână ascunsă. Start: 5 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 16 ture pe jucător decide scorul. Resurse: 12 cărți-sursă în fiecare pachet; o dată pe tură pui una (sursa i): energia +1 (maximum 8); energia se reface la fiecare tură. Fiecare are 3 zone cu 8 viață; o unitate lovește o zonă (atac k zN); cu două zone distruse pierzi. Până la 5 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, zbor, primul, armură, venin); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: lovitură, pescuit, nimicire, întoarcere în mână, întărire. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Expediția Aurie, J2 Șacalii Cenușii; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți, plus 12 cărți-sursă).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Hamal de Torțe | 1 | 1/2 | — |
| Cercetător Zelos | 1 | 1/1 | primul |
| Cățărător Iute | 2 | 2/2 | zbor |
| Pușcaș de Capcană | 2 | 3/2 | — |
| Hoț de Morminte | 3 | 5/1 | elan |
| Cartograf Neînfricat | 3 | 4/3 | — |
| Paznic Împietrit | 4 | 5/3 | elan |
| Mumie Încătușată | 4 | 6/3 | — |
| Gardian de Sarcofag | 5 | 6/3 | venin |
| Regele Îngropat | 6 | 7/5 | armură |

| Vrajă | Cost | Efect |
|---|---|---|
| Aruncare de Cuțit | 3 | lovitură 4 |
| Hartă Veche | 2 | pescuit 2 |
| Prăbușirea Tavanului | 4 | nimicire 9 |
| Cheie Fermecată | 2 | întoarcere în mână 0 |
| Frânghie și Cârlig | 3 | întărire 3 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
