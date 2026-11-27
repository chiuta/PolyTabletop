# Comando de elită (`comando_de_elita`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play comando_de_elita --seed 1`. Sursa: `polytabletop-games/src/comando_de_elita.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Echipa Albastră contra Rețeaua Roșie), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 14 ture pe jucător decide scorul. O dată pe tură poți pune orice carte din mână drept sursă (sursa i): energia +1 (maximum 8), dar cartea se pierde; energia se reface la fiecare tură. Câștigi când viața adversarului ajunge la 0 (start 22); lovitura directă scade viața cu atacul unității. Până la 6 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, zbor, primul, armură, venin); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: lovitură, măturare, nimicire, întărire, întoarcere în mână. Răsucire: loviturile directe sunt cu 1 mai tari. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Echipa Albastră, J2 Rețeaua Roșie; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Soldat Cercetaș | 1 | 2/1 | — |
| Pușcaș de Elită | 1 | 1/1 | primul |
| Medic de Teren | 2 | 2/2 | elan |
| Parașutist | 2 | 2/3 | — |
| Tunar Greu | 3 | 4/2 | zbor |
| Specialist în Demolări | 3 | 4/2 | armură |
| Pilot de Elicopter | 4 | 7/2 | — |
| Căpitan de Echipă | 4 | 3/4 | venin |
| Tanc Ușor | 5 | 8/3 | — |
| General Cu Ochelari | 6 | 7/5 | primul |

| Vrajă | Cost | Efect |
|---|---|---|
| Foc de Acoperire | 3 | lovitură 4 |
| Grenadă | 3 | măturare 1 |
| Parașutare | 4 | nimicire 9 |
| Încărcător Nou | 1 | întărire 1 |
| Asalt General | 2 | întoarcere în mână 0 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
