# Umbre de neon (`umbre_de_neon`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play umbre_de_neon --seed 1`. Sursa: `polytabletop-games/src/umbre_de_neon.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Gașca Străzii contra Corporația Aurie), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 16 ture pe jucător decide scorul. O dată pe tură poți pune orice carte din mână drept sursă (sursa i): energia +1 (maximum 8), dar cartea se pierde; energia se reface la fiecare tură. Câștigi când viața adversarului ajunge la 0 (start 20); lovitura directă scade viața cu atacul unității. Până la 5 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, zbor, primul, venin, sorbire); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: lovitură, îngheț, întoarcere în mână, întărire, măturare. Răsucire: loviturile directe sunt cu 1 mai tari. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Gașca Străzii, J2 Corporația Aurie; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Curier pe Bicicletă | 1 | 2/1 | — |
| Hoț de Buzunar | 1 | 2/1 | — |
| Hacker de Cartier | 2 | 4/1 | — |
| Samurai de Stradă | 2 | 3/1 | primul |
| Șaman Urban | 3 | 5/1 | elan |
| Mercenar Cromat | 3 | 5/1 | zbor |
| Drone-Căpitan | 4 | 4/4 | sorbire |
| Fixer Șiret | 4 | 4/3 | venin |
| Asasin Fantomă | 5 | 7/3 | elan |
| Dragonul Din Turn | 6 | 9/4 | — |

| Vrajă | Cost | Efect |
|---|---|---|
| Glonț Fantomă | 1 | lovitură 2 |
| Mască Falsă | 1 | îngheț 0 |
| Ținta Pierdută | 1 | întoarcere în mână 0 |
| Injecție | 2 | întărire 2 |
| Pană de Curent | 5 | măturare 3 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
