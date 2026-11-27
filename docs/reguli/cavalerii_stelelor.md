# Cavalerii stelelor (`cavalerii_stelelor`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play cavalerii_stelelor --seed 1`. Sursa: `polytabletop-games/src/cavalerii_stelelor.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Ordinul Strălucitor contra Cetatea Neagră), mână ascunsă. Start: 5 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 8, și se reface la fiecare tură. Câștigi când viața adversarului ajunge la 0 (start 20); lovitura directă scade viața cu atacul unității. Până la 5 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (gardă, zbor, primul, armură, sorbire); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: lovitură, binecuvântare, întărire, vindecare, măturare. Răsucire: loviturile directe sunt cu 1 mai tari. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Ordinul Strălucitor, J2 Cetatea Neagră; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Scutier Silitor | 1 | 2/1 | — |
| Lăncier Spațial | 1 | 1/1 | primul |
| Pilot de Recunoaștere | 2 | 2/2 | zbor |
| Duelist de Gardă | 2 | 1/4 | — |
| Cavaler Mărunt | 3 | 3/3 | sorbire |
| Paznic de Pod | 3 | 3/4 | gardă |
| Mistic Tăcut | 4 | 4/4 | armură |
| Maestru-Cavaler | 4 | 4/5 | — |
| Căpitan Înzăuat | 5 | 7/4 | — |
| Marele Magistru | 6 | 5/7 | primul |

| Vrajă | Cost | Efect |
|---|---|---|
| Lamă Spirituală | 2 | lovitură 3 |
| Pavăza Credinței | 2 | binecuvântare 1 |
| Salt de Forță | 2 | întărire 2 |
| Cântec Calm | 1 | vindecare 1 |
| Furtună de Stele | 4 | măturare 2 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
