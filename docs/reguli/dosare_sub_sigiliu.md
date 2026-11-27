# Dosare sub sigiliu (`dosare_sub_sigiliu`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play dosare_sub_sigiliu --seed 1`. Sursa: `polytabletop-games/src/dosare_sub_sigiliu.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Biroul Anchetelor contra Agenția Tăcută), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 16 ture pe jucător decide scorul. Resurse: 11 cărți-sursă în fiecare pachet; o dată pe tură pui una (sursa i): energia +1 (maximum 9); energia se reface la fiecare tură. Fiecare are 3 zone cu 6 viață; o unitate lovește o zonă (atac k zN); cu două zone distruse pierzi. Până la 5 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (gardă, zbor, primul, armură, venin); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: lovitură, pescuit, nimicire, vindecare, îngheț. Răsucire: cine ucide o unitate în luptă trage o carte. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Biroul Anchetelor, J2 Agenția Tăcută; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți, plus 11 cărți-sursă).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Stagiar Curios | 1 | 2/1 | — |
| Arhivist | 2 | 3/2 | — |
| Fotograf de Teren | 2 | 2/2 | primul |
| Interpret de Semnale | 3 | 2/5 | — |
| Agent Dublu | 3 | 1/6 | gardă |
| Expert în Neexplicabil | 4 | 2/7 | gardă |
| Șef de Birou | 4 | 2/6 | zbor |
| Cercetător Dispărut | 5 | 3/6 | venin |
| Martor Ascuns | 6 | 6/7 | — |
| Directorul Din Umbră | 7 | 6/8 | armură |

| Vrajă | Cost | Efect |
|---|---|---|
| Interogatoriu | 1 | lovitură 2 |
| Dosar Deschis | 2 | pescuit 2 |
| Dispariție | 3 | nimicire 5 |
| Cură de Odihnă | 3 | vindecare 2 |
| Mandat de Căutare | 2 | îngheț 0 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
