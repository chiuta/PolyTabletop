# Mașinile orei zero (`masini_ceas_zero`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play masini_ceas_zero --seed 1`. Sursa: `polytabletop-games/src/masini_ceas_zero.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Rezistența Subterană contra Rețeaua de Oțel), mână ascunsă. Start: 5 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 14 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 10, și se reface la fiecare tură. Câștigi când viața adversarului ajunge la 0 (start 24); lovitura directă scade viața cu atacul unității. Până la 6 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, gardă, zbor, armură, venin); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: lovitură, nimicire, măturare, binecuvântare, îngheț. Răsucire: daunele nu se vindecă niciodată. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Rezistența Subterană, J2 Rețeaua de Oțel; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Scrutător Rapid | 1 | 2/1 | — |
| Dronă de Pază | 1 | 1/2 | gardă |
| Infiltrator Mecanic | 2 | 3/2 | — |
| Vânător de Cabluri | 2 | 2/2 | zbor |
| Tanc Păianjen | 3 | 4/2 | elan |
| Soldat Cromat | 3 | 4/2 | armură |
| Distrugător de Bastion | 4 | 4/4 | armură |
| Comandant Metalic | 4 | 6/3 | — |
| Turn de Foc | 5 | 7/4 | — |
| Nucleul Neînvins | 6 | 7/4 | venin |

| Vrajă | Cost | Efect |
|---|---|---|
| Fascicul de Plasmă | 2 | lovitură 3 |
| Ucidere Țintită | 4 | nimicire 9 |
| Salt în Timp | 4 | măturare 2 |
| Supraîncărcare | 2 | binecuvântare 1 |
| Gaură Neagră | 1 | îngheț 0 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
